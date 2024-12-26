# image_dups_rust
Image duplicate finder written in Rust

This utility hashes images within a given directory, compares the hash distance, and returns a list of images that are visually "close enough" to be considered duplicates.

The tests directory contains some examples of images.

A similar system was used to detect wasteful texture data in the game Starfield (however that system was written in Python).
This was created to dip my toe into Rust, apologies for code that isn't idiomatic or is otherwise generally terrible.

# Build

Requires the [Rust toolchain](https://www.rust-lang.org/tools/install) to build.
Once installed you can clone the repot and run `cargo test` to run tests.

# Usage
Running the binary with `-h` shows options. If running through Cargo, finding duplicates in the test directory is something like:

`cargo run --release -- -d "tests" -r -s 16 -m 8 -o output.txt`

This finds duplicates in the "tests" directory, recursively, with a hash size of 16, a minimum distance of 8, and outputs the result to output.txt.
Note that minimum distance is related to hash size. A distance of 8 on a hash size of 16 will capture images that are similar but not identical. A minimum of 0 will capture only images that have totally identical hashes.
