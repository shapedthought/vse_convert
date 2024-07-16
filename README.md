[![Rust](https://github.com/shapedthought/vse_convert/actions/workflows/rust.yml/badge.svg)](https://github.com/shapedthought/vse_convert/actions/workflows/rust.yml)

# vse convert

VSE Convert takes a workload.txt file from the original / classic https://vse.veeambp.com sizer and
converts it into the new VSE JSON format https://calculators.veeam.com/vse

Updated 150724 to work with the newer VSE format which includes NAS. 

A new feature has been added that lets you select one or more worklaods to be NAS.

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
  -n, --nas <NAS>              NAS Workload names
  -p, --print                  Print the result
  -h, --help                   Print help
  -V, --version                Print version
```

Example 1 - no NAS:

    vse_convert -v workload.txt -s vse_new_out

Example 2 - with NAS

    vse_convert -n 'workload1','workload2' -v workload.txt -s vse_new_out

Note that the delimiter between the workload names is a comma. 

You do not need to include .json at the end of the output file name.