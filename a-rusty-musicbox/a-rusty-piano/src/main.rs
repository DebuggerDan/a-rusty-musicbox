//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-musicbox Project - Piano Main Wrapper Implementation File
//! Dan Jang, 12/05/2023
//! Note: After spending most of my energy on the Theremin instrument, I was approaching the time limits;
//! ...but I did not want to simply leave a-rusty-piano completely blank, so I thought to at least implement
//! ...a very simple wrapper piano library for the piano_rs crate (https://crates.io/crates/piano-rs) by Ritiek M. [MIT License]

use piano_rs;

/// Wrapper direct-call function for piano_rs main function:
fn main() {
    piano_rs::main();
}
