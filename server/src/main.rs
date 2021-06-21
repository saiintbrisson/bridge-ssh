#[macro_use]
extern crate log;

mod server;
mod session;
mod settings;

use server::Server;
use tokio::runtime::Builder as RuntimeBuilder;

use crate::settings::Settings;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("LOG_LEVEL")
        .init();

    let settings: Settings = Settings::new()?;
    let keys = crate::settings::load_keys(settings.keys_dir())?;

    info!("loaded {} host keys", keys.len());

    RuntimeBuilder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(server::init_server(Server::new(settings, keys)))
}
