use flate2::read::MultiGzDecoder;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(input_file: &str, format: &str) -> Result<(), Box<dyn Error>> {
    let reader = open_file(input_file).expect(&format!("Could not open {}", input_file));

    let mut read_id = String::new();
    let mut sequence = String::new();

    let mut hash_info: HashMap<String, usize> = HashMap::new();
    let mut num_read: usize = 0;
    let mut dup_read: usize = 0;

    if format == "fastq" {
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            match i % 4 {
                0 => read_id = line,
                1 => sequence = line,
                _ => (),
            }
            if i % 4 == 3 {
                let read_id = &read_id[1..];
                let sequence_length = sequence.len();

                match hash_info.entry(read_id.to_string()) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(sequence_length);
                        num_read += 1;
                    }
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        num_read += 1;
                        dup_read += 1;
                    }
                }
            }
        }
    } else if format == "fasta" {
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            match i % 2 {
                0 => read_id = line,
                1 => sequence = line,
                _ => (),
            }
            if i % 2 == 1 {
                let read_id = read_id.trim_start_matches('@');
                let sequence_length = sequence.len();

                match hash_info.entry(read_id.to_string()) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(sequence_length);
                        num_read += 1;
                    }
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        num_read += 1;
                        dup_read += 1;
                    }
                }
            }
        }
    }

    let values: Vec<_> = hash_info.values().collect();
    let max_value: isize = match values.iter().max() {
        Some(value) => **value as isize,
        None => -1,
    };
    let min_value: isize = match values.iter().min() {
        Some(value) => **value as isize,
        None => -1,
    };
    let sum: f64 = values.iter().map(|&x| *x as f64).sum();
    let average = sum / values.len() as f64;

    let mut sorted_values = values.clone();
    sorted_values.sort();
    let median = if values.len() / 2 % 2 == 0 {
        *sorted_values[values.len() / 2] as f64
    } else {
        (*sorted_values[values.len() / 2 - 1] as f64 + *sorted_values[values.len() / 2] as f64)
            / 2 as f64
    };

    let mut len: usize = 0;
    let total = sum as usize;
    let mut n50 = 0;

    for val in sorted_values.iter() {
        len += **val;
        if len > total / 2 {
            n50 = **val;
            break;
        }
    }

    println!("Total n:     {}", num_read);
    println!("Duplicate n: {}", dup_read);
    println!("Total seq:   {} bp", total);
    println!("Avg. seq:    {} bp", average);
    println!("Median seq:  {} bp", median);
    println!("N50:         {} bp", n50);
    println!("Min seq:     {} bp", min_value);
    println!("Max seq:     {} bp", max_value);
    Ok(())
}

fn open_file<P: AsRef<Path>>(p: P) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    let r = File::open(p.as_ref())?;
    let ext = p.as_ref().extension();

    if ext == Some(std::ffi::OsStr::new("gz")) {
        let gz = MultiGzDecoder::new(r);
        let buf_reader = BufReader::new(gz);
        Ok(Box::new(buf_reader))
    } else {
        let buf_reader = BufReader::new(r);
        Ok(Box::new(buf_reader))
    }
}
