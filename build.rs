use std::path;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssd: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    if !path::Path::new("cfg.toml").exists() {
        anyhow::bail!("Wi-Fi credentials missing. See cfg.toml.");
    }

    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

    Ok(())
}
