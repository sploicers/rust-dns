mod parser;
mod server;

use log::info;
use server::DnsResolver;
use std::error::Error;

const SERVER_PORT: u16 = 8000;

fn main() -> Result<(), Box<dyn Error>> {
    initialize_logger();
    let mut resolver = DnsResolver::new(SERVER_PORT)?;

    info!("Server listening on port {}.", SERVER_PORT);
    resolver.start_listening()?;

    info!("Server shutting down.");
    Ok(())
}

fn initialize_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Stdout);
    builder.init();
}
