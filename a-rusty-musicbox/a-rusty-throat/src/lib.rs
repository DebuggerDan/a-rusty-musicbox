//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-throat - Throat Library Implementation File (lib.rs)
//! Dan Jang, 12/05/2023

mod larynx;
mod vocalcords;

use cpal::traits::StreamTrait;

/// Main function for A Rusty Throat!
pub fn sing() -> Result<(), anyhow::Error> {
    // *cough cough* initialize le voice
    let rate = 48000;
    let trachea = vocalcords::Trachea::new(rate);
    let voice = trachea.shazam();

    let speech = trachea
        .talk()
        .expect("[A Rusty Throat - Main Larynx - Part 1]: Sore throat?");
    speech
        .play()
        .expect("[A Rusty Throat - Main Larynx - Part 2]: Dry throat?");

    larynx::warm_up(voice)?;
    Ok(())
}
