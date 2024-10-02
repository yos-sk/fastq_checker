use std::collections::HashSet;
use std::error::Error;
use std::io::BufRead;
use rust_htslib::{bam, bam::Read};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use fastq_checker::open_file;

pub fn run(input_file: &str, format: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let reader = open_file(input_file).expect(&format!("Could not open {}", input_file));

    let mut read_id = String::new();
    let mut sequence = String::new();
    let mut desc = String::new();
    let mut qual = String::new();

    let mut hash_info: HashSet<String> = HashSet::new();

    if format == "bam" {
        let mut bam =
            bam::Reader::from_path(input_file).expect(&format!("Could not open {}", input_file));
        
        let header = bam::Header::from_template(bam.header());
        let mut out = bam::Writer::from_path(output_file, &header, bam::Format::Bam)
            .expect(&format!("Could not open {}", output_file));
        
        for read in bam.records().map(|r| r.expect("Failure parsing Bam file")) {
            read_id = String::from_utf8_lossy(read.qname()).to_string();
            match hash_info.get(&read_id) {
                Some(_) => {
                    continue;
                }
                None => {
                    out.write(&read).unwrap();
                    hash_info.insert(read_id.clone());
                }
            }
        }
    } else if format == "fastq" {
        let path = Path::new(output_file);
        let mut out = File::create(&path)?;
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
                        writeln!(out, "{}", read_id)?;
                        writeln!(out, "{}", sequence)?;
                        writeln!(out, "{}", desc)?;
                        writeln!(out, "{}", qual)?;
                        hash_info.insert(read_id.clone());
                    }
                }
            }
        }
    } else if format == "fasta" {
        let path = Path::new(output_file);
        let mut out = File::create(&path)?;
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
                    writeln!(out, "{}", read_id)?;
                    writeln!(out, "{}", sequence)?;
                    hash_info.insert(read_id.clone());
                }
            }
        }
    }
    Ok(())
}
