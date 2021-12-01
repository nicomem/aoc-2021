# Advent of Code 2021

Solutions to the [Advent of Code 2021](https://adventofcode.com/2021/).

## Pre-requirements

- Rust 1.57+ toolchain
  - Tested on 1.59 nightly but no nightly features are used 

## Configuration

You can configure the program either with command line arguments or by creating a `.env` file with the following entries:

```bash
# Path to the directory where the data files are, or where they will be downloaded.
# If you want to provide the data files, they must be named "dayN.zst" and be encoded in the zstd format.
DATA_PATH = ...

# AoC Cookie session identifier, used to download your user input data.
# This can be found in the Developer Tools > Network > Request Headers of your browser
# -> Cookie: session=<AOC_SESSION>
AOC_SESSION = ...
```

## Run the program

```bash
cargo run -- <DAY>
```
