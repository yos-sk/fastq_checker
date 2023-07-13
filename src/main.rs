use clap::Parser;
use std::process;

mod check;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short = 'i', long)]
    input_file: String,

    #[arg(short = 'f', long)]
    format: String,
}

fn main() {
    let arguments = Arguments::parse();
    if let Err(error) = check::run(
        &arguments.input_file,
        &arguments.format
    ) {
        eprintln!("{}", error);
        process::exit(1);
    }
}