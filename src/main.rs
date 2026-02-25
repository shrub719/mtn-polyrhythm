use clap::{ Parser, Subcommand };
use std::path::PathBuf;

mod compile;
mod pack;
mod osu;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        input: PathBuf,

        #[arg(short, long, default_value = "./output.mtb")]
        output: PathBuf
    },

    Pack {
        input: Vec<PathBuf>,

        #[arg(short, long, default_value = "./output.mtp")]
        output: PathBuf
    },

    Osu {
        input: PathBuf,

        #[arg(short, long, default_value = "./output.mtn")]
        output: PathBuf
    }
}

fn main() {
    let cli = Cli::parse();

    use Commands::*;
    match cli.command {
        Compile { input, output }=> {
            compile::compile(input, output);
        },
        Pack { input: _, output: _ }=> {
            todo!()
        },
        Osu { input, output } => {
            osu::osu(input, output);
        }
    }
}
