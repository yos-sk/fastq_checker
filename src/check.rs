use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

use fastq_checker::open_file;


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
                    std::collections::hash_map::Entry::Occupied(_entry) => {
                        num_read += 1;
                        dup_read += 1;
                    }
                }
            }
        }
    } else if format == "fasta" {
        for line in reader.lines() {
            let line = line?;
            if line.starts_with(">") {
                if !sequence.is_empty() {
                    let read_id = read_id.trim_start_matches('@');
                    let sequence_length = sequence.len();
                    match hash_info.entry(read_id.to_string()) {
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(sequence_length);
                            num_read += 1;
                        }
                        std::collections::hash_map::Entry::Occupied(_entry) => {
                            num_read += 1;
                            dup_read += 1;
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
            let read_id = read_id.trim_start_matches('@');
            let sequence_length = sequence.len();

            match hash_info.entry(read_id.to_string()) {
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(sequence_length);
                    num_read += 1;
                }
                std::collections::hash_map::Entry::Occupied(_entry) => {
                    num_read += 1;
                    dup_read += 1;
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
    let mut n90 = 0;

    for val in sorted_values.iter() {
        len += **val;
        if len > total / 2 {
            n50 = **val;
            break;
        }
    }

    for val in sorted_values.iter() {
        len += **val;
        if len > total * 9 / 10 {
            n90 = **val;
            break;
        }
    }


    println!("Total_n:     {}", num_read);
    println!("Total_bp:   {}", total);
    println!("Avg_bp:    {}", average);
    println!("Median_bp:  {}", median);
    println!("N50_bp:         {}", n50);
    println!("Min_bp:     {}", min_value);
    println!("Max_bp:     {}", max_value);
    println!("Duplicate_n: {}", dup_read);
    println!("N90_bp:         {}", n90);
    Ok(())
}
