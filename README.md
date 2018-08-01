# rust-na

## Introduction

DNA-inspired structs and functions in Rust, mostly for the author's practice.

## Requirements

See `Cargo.toml`. This shouldn't be much of a concern when using Cargo, anyway.

At the moment, a nightly build is needed (for try_from). You can switch with `rustup`.

## Instructions

The executable will take a string of elements in "ACGT" and print out the protein sequence that the DNA codes for.

For instance:

```
cargo run CCATGTCGAAGTCGCTAAGCTCTCGTAGAAAATCGATTAGATAAATATATATATGCTGCTCGAGATCGA
      MMSKSLSSRRKSIR
      MMLLE
```

The `-f` option can also be passed, in which case the executable will try to open the file with the given name for reading.

```
cargo run -- -f gene.data
```
