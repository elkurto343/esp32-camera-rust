use esp_idf_hal::{gpio::PinDriver, modem::Modem, peripheral::Peripheral};
use esp_idf_svc::{
    eventloop::{EspBackgroundEventLoop, EspSystemEventLoop},
    wifi::EspWifi,
};

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    ssid: &'static str,
    #[default("")]
    pass: &'static str,
}

pub fn wifi(
    modem: impl Peripheral<P = Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<Box<EspWifi<'static>>> {
    use esp_idf_svc::handle::RawHandle;
    use std::net::Ipv4Addr;

    let config = CONFIG;

    let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), None)?);
    println!("wifi: created");

    let ap_infos = wifi.scan()?;

    println!("wifi: ap list");
    for info in ap_infos.into_iter() {
        println!("- {}", info.ssid);
    }

    let target = ap_infos.into_iter().find(|ap| ap.ssid == config.ssid);
    if let Some(target) = target {
        println!("wifi: ssid matched");
        Some(target.channel)
    } else {
        println!("wifi: ssid not matched");
        None
    }

    wifi.set_configuration(&Configuration::Mixed(
            ClientConfiguration {
                ssid: config.ssid.into(),
                password: config.pass.into(),
                channel,
                ..Default::default(),
            },
            
            AccessPointConfiguration {
                ssid: "aptest".into(),
                channel: channel.unwrap_or(1),
                ..Default::default(),
            }
            ))?;

    wifi.start()?;

    if !WifiWait::new(&sysloop)?
        .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
    {
        bail!("wifi: device failed to start");

    }

    println!("wifi: connecting to {}", config.ssid);

    wifi.connect()?;

    if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
        Duration::from_secs(20),
        || {
            wifi.is_connected().unwrap()
                && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
        },
    ) {
        bail!("wifi: did not connect or did not receive a DHCP lease");
    }

    let ip_info = wifi.sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    // ping(ip_info.subnet.gateway)?;

    Ok(wifi)


}
