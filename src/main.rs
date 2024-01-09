use clap::{Parser, Subcommand};
use std::process;

mod check;
mod rmdup;
mod extract;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        #[arg(short = 'i', long)]
        input_file: String,

        #[arg(short = 'f', long)]
        format: String,
    },

    Rmdup {
        #[arg(short = 'i', long)]
        input_file: String,

        #[arg(short = 'f', long)]
        format: String,
    },
    Extract {
        #[arg(short = 'i', long)]
        input_file: String,

        #[arg(short = 'f', long)]
        format: String,

        #[arg(short = 'l', long)]
        length: usize,
    },
}

fn main() {
    let arguments = Arguments::parse();

    match &arguments.command {
        Commands::Check{input_file,
                        format} => {
            if let Err(error) = check::run(input_file, format) {
                eprintln!("{}", error);
                process::exit(1);
            }
        },

        Commands::Rmdup{input_file,
                        format} => {
            if let Err(error) = rmdup::run(input_file, format) {
                eprintln!("{}", error);
                process::exit(1);
            }
        },

        Commands::Extract{input_file,
                        format,
                        length} => {
            if let Err(error) = extract::run(input_file, format, *length) {
                eprintln!("{}", error);
                process::exit(1);
            }
        },
    }
}
