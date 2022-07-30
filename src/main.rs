mod command_line;
mod parser;
mod resolver;

use command_line::{parse_cli_args, PacketOperation};
use parser::DnsPacket;
use resolver::DnsResolver;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    net::Ipv4Addr,
};

const RESOLVER_PORT: u16 = 8000;

fn main() -> Result<(), Box<dyn Error>> {
    match parse_cli_args() {
        PacketOperation::Decode { infile, outfile } => {
            let mut packet = DnsPacket::from_reader(&mut get_packet_reader(infile)?)?;
            packet.write_direct(&mut get_packet_writer(outfile)?)?;
        }
        PacketOperation::Query { infile, outfile } => {
            let resolver = DnsResolver::new(Ipv4Addr::new(8, 8, 8, 8), RESOLVER_PORT)?;
            let mut query_packet = DnsPacket::from_reader(&mut get_packet_reader(infile)?)?;
            let mut response_packet = resolver.query(&mut query_packet)?;
            response_packet.write_direct(&mut get_packet_writer(outfile)?)?;
        }
        _ => (),
    }
    Ok(())
}

pub fn get_packet_reader(filename: Option<String>) -> Result<Box<dyn Read>, Box<dyn Error>> {
    match filename {
        Some(f) => Ok(Box::new(File::open(f)?)),
        None => Ok(Box::new(std::io::stdin().lock())),
    }
}

pub fn get_packet_writer(filename: Option<String>) -> Result<Box<dyn Write>, Box<dyn Error>> {
    match filename {
        Some(f) => Ok(Box::new(File::open(f)?)),
        None => Ok(Box::new(std::io::stdout().lock())),
    }
}
