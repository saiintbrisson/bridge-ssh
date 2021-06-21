#[macro_use]
extern crate log;

mod settings;

use crate::settings::Settings;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("LOG_LEVEL")
        .init();
    let settings: Settings = Settings::new()?;
    let keys = crate::settings::load_keys(settings.keys_dir())?;

    info!("loaded {} host keys", keys.len());
}
