# fastq_checker
Check duplicates and calculate statistics

## Install

```
git clone https://github.com/yos-sk/fastq_checker.git
cd fastq_checker
cargo build --release
```

## Usage

```
./target/release/fastq_checker --input-file ${INPUT_FILE} --format {fastq/fasta}
```