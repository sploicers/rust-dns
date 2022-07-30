mod parser;
mod resolver;

use resolver::DnsResolver;
use std::{error::Error, net::Ipv4Addr};

const RESOLVER_PORT: u16 = 8000;

fn main() -> Result<(), Box<dyn Error>> {
    let resolver = DnsResolver::new(Ipv4Addr::new(8, 8, 8, 8), RESOLVER_PORT)?;
    resolver.start_listening()?;
    Ok(())
}
