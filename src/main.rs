use clap::Parser;

mod binary;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    input: std::path::PathBuf,

    #[arg(short, long, default_value = "./output.mtb")]
    output: std::path::PathBuf
}

fn main() {
    let args = Args::parse();

    binary::convert_map(args.input, args.output);
}
