# fastq_checker
Check sequence duplicates and calculate sequence statistics from bam/fasta/fastq

## Install

```
git clone https://github.com/yos-sk/fastq_checker.git
cd fastq_checker
cargo install --path . 
```

## Usage

```
Usage: fastq_checker <COMMAND>

Commands:
  check
  rmdup
  extract
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### check
This command checks read duplicates and calculate statistics from fasta/fastq/bam file
```
fastq_checker check -h
Usage: fastq_checker check --input-file <INPUT_FILE> --format <FORMAT>

Options:
  -i, --input-file <INPUT_FILE>
  -f, --format <FORMAT>
  -h, --help                     Print help
```

### rmdup
This command removes read duplicates in fasta/fastq/bam file
```
fastq_checker rmdup -h
Usage: fastq_checker rmdup --input-file <INPUT_FILE> --format <FORMAT> --output-file <OUTPUT_FILE>

Options:
  -i, --input-file <INPUT_FILE>
  -f, --format <FORMAT>
  -o, --output-file <OUTPUT_FILE>
  -h, --help                       Print help
```

### extract
This command extracts reads longer than the specified length from fasta/fastq/bam file
```
fastq_checker extract -h
Usage: fastq_checker extract --input-file <INPUT_FILE> --format <FORMAT> --length <LENGTH>

Options:
  -i, --input-file <INPUT_FILE>
  -f, --format <FORMAT>
  -l, --length <LENGTH>
  -h, --help                     Print help
```
