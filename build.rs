use std::path;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    ssid: &'static str,
    #[default("")]
    pass: &'static str,
}

fn main() -> anyhow::Result<()> {
    if !path::Path::new("cfg.toml").exists() {
        anyhow::bail!("Wi-Fi credentials missing. See `cfg.toml.example`.");
    }

    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

    Ok(())
}
