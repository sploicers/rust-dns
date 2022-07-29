mod parser;
mod resolver;

use parser::DnsPacket;
use resolver::DnsResolver;
use std::{error::Error, fs::File, io::Read, net::Ipv4Addr};

const RESOLVER_PORT: u16 = 8000;

fn main() -> Result<(), Box<dyn Error>> {
    print!("{}", resolve_query()?);
    Ok(())
}

fn resolve_query() -> Result<DnsPacket, Box<dyn Error>> {
    let resolver = DnsResolver::new(Ipv4Addr::new(8, 8, 8, 8), RESOLVER_PORT)?;
    resolver.query(&mut decode_packet()?)
}

fn decode_packet() -> Result<DnsPacket, Box<dyn Error>> {
    DnsPacket::from_reader(&mut get_packet_reader()?)
}

fn get_packet_reader() -> Result<Box<dyn Read>, Box<dyn Error>> {
    let mut filename = String::new();
    Ok(if got_filename_from_args(&mut filename) {
        Box::new(File::open(filename)?)
    } else {
        Box::new(std::io::stdin().lock())
    })
}

fn got_filename_from_args(filename: &mut String) -> bool {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.len() {
        1 => {
            filename.push_str(&args[0]);
            true
        }
        _ => false,
    }
}
