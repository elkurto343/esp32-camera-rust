fn main() -> anyhow::Result<()> {
    // TODO: use args instead of env ?
    dotenv_build::output(dotenv_build::Config::default()).unwrap();

    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

    Ok(())
}
