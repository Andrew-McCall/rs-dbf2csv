# DBF to CSV Converter

## Description
A Simple Rust executable to convert `.dbf` files (optionally associated with `.fpt` memo files) to `.csv` format using the [`dbase`](https://crates.io/crates/dbase) and [`csv`](https://crates.io/crates/csv) crates.

## Features / Notes

- Reads all `.dbf` files in the current directory.
- Supports `.fpt` memo files if present.
- Converts DBF fields to CSV with appropriate formatting.
- Outputs `.csv` files named after the input DBF files.

*(The tool does not edit or delete the source database files)*

## License
MIT License
Copyright (c) 2025 Andrew McCall
