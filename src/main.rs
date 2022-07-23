mod parser;

use parser::DnsPacket;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut filename = String::new();
    if got_filename_from_args(&mut filename) {
        print!("Reading packet from file: {}.\n\n", filename);
        print!("{}", DnsPacket::from_file(File::open(filename)?)?);
    } else {
        print!("Reading packet from standard input.\n\n");
        print!("{}", DnsPacket::from_stdin()?);
    }
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
