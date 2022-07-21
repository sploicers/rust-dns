mod parser;

use std::fs::File;
use std::io::Read;

use parser::{DnsPacket, WrappedBuffer};

fn main() -> Result<(), String> {
    let mut file = File::open("response_packet.txt")?;
    let mut buffer = WrappedBuffer::new();
    file.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;

    Ok(())
}
