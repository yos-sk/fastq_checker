# fastq_checker
Check duplicates and calculate statistics

## Install

```
git clone https://github.com/yos-sk/fastq_checker.git
cd fastq_checker
cargo build --release
```

## Usage

- Check: check duplicates and calculate statistics
```
./target/release/fastq_checker check --input-file ${INPUT_FILE} --format {fastq/fasta}
```

- Rmdup: remove duplicates
```
./target/release/fastq_checker rmdup --input-file ${INPUT_FILE} --format {fastq/fasta}
```