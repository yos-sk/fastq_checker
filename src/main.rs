use clap::{Parser, Subcommand};
use std::process;

mod check;
mod extract;
mod rmdup;

#[derive(Parser)]
#[command(author="Yoshitaka Sakamoto", version="0.3.2", about="fastq/fasta/bam tool kit", long_about = None)]
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

        #[arg(short = 'o', long)]
        output_file: String,
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
        Commands::Check { input_file, format } => {
            if let Err(error) = check::run(input_file, format) {
                eprintln!("{}", error);
                process::exit(1);
            }
        }

        Commands::Rmdup { input_file, format, output_file } => {
            if let Err(error) = rmdup::run(input_file, format, output_file) {
                eprintln!("{}", error);
                process::exit(1);
            }
        }

        Commands::Extract {
            input_file,
            format,
            length,
        } => {
            if let Err(error) = extract::run(input_file, format, *length) {
                eprintln!("{}", error);
                process::exit(1);
            }
        }
    }
}
