mod parser;

use std::fs::File;
use std::io::Read;
use std::io::Result;

use parser::{DnsPacket, WrappedBuffer};

fn main() -> Result<()> {
    let mut file = File::open("response_packet.txt")?;
    let mut buffer = WrappedBuffer::new();
    file.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;

    Ok(())
}
