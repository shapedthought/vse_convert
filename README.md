[![Rust](https://github.com/shapedthought/vse_convert/actions/workflows/rust.yml/badge.svg)](https://github.com/shapedthought/vse_convert/actions/workflows/rust.yml)

# vse convert

VSE Convert takes a workload.txt file from the original / classic https://vse.veeambp.com sizer and
converts it into the new VSE JSON format https://calculators.veeam.com/vse

### Install:

1. Install Rust
2. Clone this repo
3. cd into the project directory
4. Compile the binary:

    cargo build --release

Or you can install it using:

    cargo install --path .

I may add this to crates.io in the future. 

### How to use

It is a very simple commandline application. 

```
Usage: vse_convert [OPTIONS] --vse-file <VSE_FILE> --save-file <SAVE_FILE>

Options:
  -v, --vse-file <VSE_FILE>    VSE workload file
  -s, --save-file <SAVE_FILE>  The new file name, without extension
  -p, --print                  Print the result
  -h, --help                   Print help
  -V, --version                Print version
```

Example:

    vse_convert -v workload.txt -s vse_new_out

You do not need to include .json at the end of the output file name.