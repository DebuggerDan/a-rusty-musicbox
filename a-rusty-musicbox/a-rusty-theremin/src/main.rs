//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-musicbox Project - Theremin Main Implementation File
//! Dan Jang, 12/05/2023

mod etherphone;
mod panel;

use cpal::traits::StreamTrait;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

/// The main wrapper-function for A Rusty Theremin musical instrument module-library implementation!
fn main() -> Result<(), anyhow::Error> {
    // Default frequency is set to waaaaaaay below A4, which would have been at 440 Hz,
    // ...but, for the sake for audio safety & the sanity of the user's ears, the default freq is set to a much softer 127 Hz!
    let freq = Arc::new(AtomicUsize::new(127));
    let music = etherphone::Etherphonics::new(Arc::clone(&freq));

    let performance = music.plug_in()?;
    performance.play()?;

    panel::power_on(freq)?;
    Ok(())
}
