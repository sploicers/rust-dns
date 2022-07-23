mod parser;

use parser::DnsPacket;
use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut filename = String::new();

    let mut reader: Box<dyn Read> = if got_filename_from_args(&mut filename) {
        Box::new(File::open(filename)?)
    } else {
        Box::new(std::io::stdin().lock())
    };

    print!("{}", DnsPacket::from_reader(&mut reader)?);
    Ok(())
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
