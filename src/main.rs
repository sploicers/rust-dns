mod parser;
mod server;

use server::DnsResolver;
use std::error::Error;

const SERVER_PORT: u16 = 8000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut resolver = DnsResolver::new(SERVER_PORT)?;
    resolver.start_listening()?;
    Ok(())
}
