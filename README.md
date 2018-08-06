# rust-na

## Introduction

DNA-inspired structs and functions in Rust, mostly for the author's practice.

## Requirements

See `Cargo.toml`. This shouldn't be much of a concern when using Cargo, anyway.

At the moment, a nightly build is needed (for try_from). You can switch with `rustup`.

## Instructions

With a "-i" flag, the executable will take a string of elements in "ACGT" and print out the protein sequence that the DNA codes for.

For instance:

```
cargo run -- -i CCATGTCGAAGTCGCTAAGCTCTCGTAGAAAATCGATTAGATAAATATATATATGCTGCTCGAGATCGA
      MSKSLSSRRKSIR
      MLLEI
```

If no flag option is passed, the executable will try to open the file(s) with the given name for reading.

```
cargo run gene.data
```

## TODO

fix file reading process to deal with standard (and possibly large) FASTA files intelligently

function to copy/mutate a BaseSeq with configurable insertion, deletion, and SNP rates.

sexual reproduction with crossover

comparison functions of sequences (e.g. total base numbers, "strength" (GC minus AT) or other combinations). can do di/tri-nucleotide frequency and correlation functions as well. this could help form metrics for sequence comparison.
