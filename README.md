An experimental make implementation in Rust

### Goals
Writing a make sounds kinda fun, which also gives me an opportunity to familiarize myself with async Rust.

Regarding the features/syntax: I'm trying to get it compatible with GNU Make 3.81 as shipped on MacOS, newer features might be worked on as well but the Guile integration is certainly out-of-scope :D

For now I'll just add features to get my own C projects to compile via rmake while ignoring performance, making sure to write lots of tests so that after things stabilize optimizing becomes easier.