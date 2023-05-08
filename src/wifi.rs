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
) -> anyhow::Result<()> {
    /* -> anyhow::Result<Box<EspWifi<'static>>> */
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

    // TODO: connect to SSID
    println!("wifi: connecting to {}", config.ssid);

    Ok(())
}
