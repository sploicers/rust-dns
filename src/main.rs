mod parser;

use std::fs::File;
use std::io::Read;

use parser::{DnsPacket, WrappedBuffer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("response_packet.txt")?;
    let mut buffer = WrappedBuffer::new();
    file.read(&mut buffer.buf)?;

    if let Ok(packet) = DnsPacket::from_buffer(&mut buffer) {
        print!("{:?}", packet.header);
    }
    Ok(())
}
