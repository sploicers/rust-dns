use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CommandLineArgs {
    #[clap(short, long, action, group("operation"))]
    decode: bool,

    #[clap(short, long, action, group("operation"))]
    resolve: bool,

    #[clap(short, long, value_parser, value_name = "INPUT FILE")]
    file: Option<String>,

    #[clap(short, long, value_parser, value_name = "OUTPUT FILE")]
    output_file: Option<String>,
}

pub fn parse_cli_args() -> PacketOperation {
    match CommandLineArgs::parse() {
        CommandLineArgs {
            decode: true,
            file,
            output_file,
            ..
        } => PacketOperation::Decode {
            infile: file,
            outfile: output_file,
        },
        CommandLineArgs {
            resolve: true,
            file,
            output_file,
            ..
        } => PacketOperation::Query {
            infile: file,
            outfile: output_file,
        },
        _ => PacketOperation::NOOP,
    }
}

#[derive(Debug)]
pub enum PacketOperation {
    Decode {
        infile: Option<String>,
        outfile: Option<String>,
    },
    Query {
        infile: Option<String>,
        outfile: Option<String>,
    },
    NOOP,
}

#[cfg(test)]
mod tests {
    use super::CommandLineArgs;
    use clap::CommandFactory;

    #[test]
    fn can_parse_cli_args() {
        CommandLineArgs::command().debug_assert()
    }
}
