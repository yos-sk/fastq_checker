use rust_htslib::{bam, bam::Read};
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

use fastq_checker::open_file;

pub fn run(input_file: &str, format: &str) -> Result<(), Box<dyn Error>> {
    let mut lengths: HashMap<String, usize> = HashMap::new();
    let mut qualities: HashMap<String, f64> = HashMap::new();
    let mut num_read: usize = 0;
    let mut dup_read: usize = 0;
    let mut num_read_ov100k: usize = 0; 

    if format == "bam" {
        let mut bam =
            bam::Reader::from_path(input_file).expect(&format!("Could not open {}", input_file));
        for read in bam.records().map(|r| r.expect("Failure parsing Bam file")) {
            if read.is_supplementary() | read.is_secondary() {
                continue;
            }
            let read_id: String = String::from_utf8_lossy(read.qname()).to_string();
            let sequence_length = read.seq_len();
            let sequence_quality: Vec<u8> = read.qual().to_vec();
            let sum_quality: usize = sequence_quality.iter().map(|&x| x as usize).sum();
            if sequence_length > 100000 {
                num_read_ov100k += 1;
            }
            if !lengths.contains_key(&read_id) {
                lengths.insert(read_id.clone(), sequence_length);
                num_read += 1;
            } else {
                num_read += 1;
                dup_read += 1;
            }

            if !qualities.contains_key(&read_id) {
                qualities.insert(read_id.clone(), sum_quality as f64 / sequence_length as f64);
                eprintln!("{}\t{}\t{}", read_id.clone(), sequence_length, sum_quality as f64 / sequence_length as f64);
            }
        }
    } else {
        let reader = open_file(input_file).expect(&format!("Could not open {}", input_file));
        let mut read_id = String::new();
        let mut sequence = String::new();
        if format == "fastq" {
            for (i, line) in reader.lines().enumerate() {
                let line = line?;
                match i % 4 {
                    0 => read_id = line.trim_start_matches('@').to_string(),
                    1 => sequence = line,
                    3 => {
                        let quality = line;
                        //let t_read_id = read_id.clone();
                        let sequence_length = sequence.len();
                        let sum_quality: usize = decode_quality(&quality).iter().sum();

                        if sequence_length > 100000 {
                            num_read_ov100k += 1;
                        }

                        if !lengths.contains_key(&read_id) {
                            lengths.insert(read_id.clone(), sequence_length);
                            num_read += 1;
                        } else {
                            num_read += 1;
                            dup_read += 1;
                        }

                        if !qualities.contains_key(&read_id) {
                            qualities.insert(read_id.clone(), sum_quality as f64 / sequence_length as f64);
                            eprintln!("{}\t{}\t{}", read_id.clone(), sequence_length, sum_quality as f64 / sequence_length as f64);
                        }
                    }
                    _ => (),
                }
            }
        } else if format == "fasta" {
            for line in reader.lines() {
                let line = line?;
                if line.starts_with(">") {
                    if !sequence.is_empty() {
                        let read_id = read_id.trim_start_matches('@').to_string();
                        let sequence_length = sequence.len();
                        if sequence_length > 100000 {
                            num_read_ov100k += 1;
                        }
                        if !lengths.contains_key(&read_id) {
                            lengths.insert(read_id.clone(), sequence_length);
                            eprintln!("{}\t{}", read_id.clone(), sequence_length);
                            num_read += 1;
                        } else {
                            num_read += 1;
                            dup_read += 1;
                        }
                        sequence.clear();
                    }
                    read_id = line;
                } else {
                    sequence.push_str(&line);
                }
            }

            if !sequence.is_empty() {
                let read_id = read_id.trim_start_matches('@').to_string();
                let sequence_length = sequence.len();

                if sequence_length > 100000 {
                    num_read_ov100k += 1;
                }

                if !lengths.contains_key(&read_id) {
                    lengths.insert(read_id.clone(), sequence_length);
                    eprintln!("{}\t{}", read_id.clone(), sequence_length);
                    num_read += 1;
                } else {
                    num_read += 1;
                    dup_read += 1;
                }
            }
        }
    }

    let values: Vec<usize> = lengths.values().cloned().collect();
    let max_value: isize = match values.iter().max() {
        Some(value) => *value as isize,
        None => -1,
    };
    let min_value: isize = match values.iter().min() {
        Some(value) => *value as isize,
        None => -1,
    };
    let sum: usize = values.iter().sum();
    let average = sum as f64 / values.len() as f64;

    let mut sorted_values = values.clone();
    sorted_values.sort();
    let median = if values.len() / 2 % 2 == 0 {
        sorted_values[values.len() / 2] as f64
    } else {
        (sorted_values[values.len() / 2 - 1] as f64 + sorted_values[values.len() / 2] as f64)
            / 2 as f64
    };

    let sum_ov100k: usize = lengths.values().filter(|&&v| v > 100000).sum();

    let mut len: usize = 0;
    let total = sum as usize;
    let mut n50 = 0;
    let mut n90 = 0;


    for val in sorted_values.iter() {
        len += *val;
        if len > total / 2 {
            n50 = *val;
            break;
        }
    }

    for val in sorted_values.iter() {
        len += *val;
        if len > total * 9 / 10 {
            n90 = *val;
            break;
        }
    }

    let q_values: Vec<f64> = qualities.values().cloned().collect();
    let q_sum: f64 = q_values.iter().sum();
    let q_len: f64 = q_values.len() as f64;
    let mean_qvalues = q_sum / q_len;

    println!("Total_n:          {}", num_read);
    println!("Total_n >100 kb   {}", num_read_ov100k);
    println!("Total_bp:         {}", total);
    println!("Total_bp >100 kb: {}", sum_ov100k);
    println!("Avg_bp:           {}", average);
    println!("Median_bp:        {}", median);
    println!("N50_bp:           {}", n50);
    println!("Min_bp:           {}", min_value);
    println!("Max_bp:           {}", max_value);
    println!("Mean_quals        {}", mean_qvalues);
    println!("N90_bp:           {}", n90);
    println!("Duplicate_n:      {}", dup_read);
    Ok(())
}


fn decode_quality(quality_str: &str) -> Vec<usize> {
    quality_str.chars().map(|c| c as usize - 33).collect()
}
