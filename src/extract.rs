use std::error::Error;
use std::io::BufRead;

use fastq_checker::open_file;

pub fn run(input_file: &str, format: &str, length: usize) -> Result<(), Box<dyn Error>> {
    let reader = open_file(input_file).expect(&format!("Could not open {}", input_file));

    let mut read_id = String::new();
    let mut sequence = String::new();
    let mut quality = String::new();

    if format == "fastq" {
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            match i % 4 {
                0 => read_id = line,
                1 => sequence = line,
                3 => quality = line,
                _ => (),
            }
            if i % 4 == 3 {
                if sequence.len() > length {
                    println!("{}", read_id);
                    println!("{}", sequence);
                    println!("+");
                    println!("{}", quality);
                }
            }
        }
    } else if format == "fasta" {
        for line in reader.lines() {
            let line = line?;
            if line.starts_with(">") {
                if !sequence.is_empty() {
                    if sequence.len() > length {
                        println!("{}", read_id);
                        println!("{}", sequence);
                    }
                }
                read_id = line;
            } else {
                sequence.push_str(&line);
            }
        }

        if !sequence.is_empty() {
            if sequence.len() > length {
                println!("{}", read_id);
                println!("{}", sequence);
            }
        }
    }
    Ok(())
}
