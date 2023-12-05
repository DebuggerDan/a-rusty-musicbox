//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-musicbox Project - Theremin Main Implementation File
//! Dan Jang, 12/05/2023

mod etherphone;
mod panel;

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use cpal::traits::StreamTrait;

/// The main wrapper-function for A Rusty Theremin musical instrument module-library implementation!
fn main() -> Result<(), anyhow::Error> {
    // Default frequency is set to A4 (440 Hz)
    let freq = Arc::new(AtomicUsize::new(440));
    let music = etherphone::Etherphonics::new(Arc::clone(&freq));

    let performance = music.plug_in()?;
    performance.play()?;

    panel::power_on(freq)?;
    Ok(())
}