mod parser;
mod server;

const SERVER_PORT: u16 = 8000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger();
    server::start_listening(SERVER_PORT)?;
    log::info!("Server shutting down.");
    Ok(())
}

fn initialize_logger() {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();
}
