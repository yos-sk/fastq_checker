use std::collections::HashSet;
use std::error::Error;
use std::io::BufRead;

use fastq_checker::open_file;

pub fn run(input_file: &str, format: &str) -> Result<(), Box<dyn Error>> {
    let reader = open_file(input_file).expect(&format!("Could not open {}", input_file));

    let mut read_id = String::new();
    let mut sequence = String::new();
    let mut desc = String::new();
    let mut qual = String::new();

    let mut hash_info: HashSet<String> = HashSet::new();

    if format == "fastq" {
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            match i % 4 {
                0 => read_id = line,
                1 => sequence = line,
                2 => desc = line,
                3 => qual = line,
                _ => (),
            }
            if i % 4 == 3 {
                match hash_info.get(&read_id) {
                    Some(_) => {
                        continue;
                    }
                    None => {
                        println!("{}", read_id);
                        println!("{}", sequence);
                        println!("{}", desc);
                        println!("{}", qual);
                        hash_info.insert(read_id.clone());
                    }
                }
            }
        }
    } else if format == "fasta" {
        for line in reader.lines() {
            let line = line?;
            if line.starts_with(">") {
                if !sequence.is_empty() {
                    match hash_info.get(&read_id) {
                        Some(_) => {
                            continue;
                        }
                        None => {
                            println!("{}", read_id);
                            println!("{}", sequence);
                            hash_info.insert(read_id.clone());
                        }
                    }
                    sequence.clear();
                }
                read_id = line;
            } else {
                sequence.push_str(&line);
            }
        }
        if !sequence.is_empty() {
            match hash_info.get(&read_id) {
                Some(_) => {
                    ();
                }
                None => {
                    println!("{}", read_id);
                    println!("{}", sequence);
                    hash_info.insert(read_id.clone());
                }
            }
        }
    }
    Ok(())
}
