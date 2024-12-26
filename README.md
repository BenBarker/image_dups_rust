# image_dups_rust
Image duplicate finder written in Rust

This utility hashes images within a given directory, compares the hash distance, and returns a list of images that are visually "close enough" to be considered duplicates.

The tests directory contains some examples of images.

A similar system was used to detect wasteful texture data in the game Starfield (however that system was written in Python).
This was created to dip my toe into Rust, apologies for code that isn't idiomatic or is otherwise generally terrible.

