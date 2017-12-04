# Advent of Code 2017

Here are my submissions for [Advent of Code 2017](https://adventofcode.com/2017). All submissions are expected to be written in [Rust](https://www.rust-lang.org/), and since I am learning it from scratch, the code quality is expected to improve as December progresses.

All scripts are formatted using `rustfmt` for increased readability.

## Rust

To install Rust, we simply use Brew.
```
$ brew install rust
```

In order to run the script files, we use `cargo-script` which is installed by

```
$ cargo install cargo-script
```

## Running tests

All tests can be run by `cargo-script`, like

```
$ cargo script --test 1/captcha.rs
```

## Running the puzzle

Since the "final" puzzle usually is slightly larger, all scripts are designed to allow input from stdin. To run a puzzle, 

```
$ cargo script 1/captcha-halfway.rs < 1/puzzle.txt
```
