//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-musicbox Project - Theremin Audio Implementation File (etherphone.rs)
//! This file contains the audio device & stream logic for the theremin audio playback,
//! ...as well as the logic for the constant generation of the theremin audio-stream - via the Etherphonics main function & heterodyne-effect simulator function!
//! Dan Jang, 12/05/2023

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Stream,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct Etherphonics {
    freq: Arc<AtomicUsize>,
    //color: ,
    clock: Arc<AtomicUsize>, // persistent clock for reducing audio lag or clocks!
}

/// The etherphone function - which was the original name for the theremin - this function initializes the theremin's audio configuration + check-processing host audio-device for playback!
impl Etherphonics {
    /// Etherphone constructor function!
    pub fn new(freq: Arc<AtomicUsize>) -> Self {
        Self {
            freq,
            clock: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Etherphone & host audio-device initialization function!
    pub fn plug_in(&self) -> Result<Stream, anyhow::Error> {
        let host = cpal::default_host();

        let speakers = host.default_output_device().ok_or(anyhow::Error::msg("[A Rusty Theremin - Etherphone]: Uh oh, it would appear that no compatible speakers/audio devices were detected?"))?;

        let settings = speakers.default_output_config()?;
        let rate = settings.sample_rate().0 as f32;
        let timeout = Some(Duration::from_millis(50));
        let channels = settings.channels() as usize;
        let freq = Arc::clone(&self.freq);
        let clock = Arc::clone(&self.clock);

        let errorz = |whoopsie| {
            eprintln!(
                "[A Rusty Theremin - Etherphone]: Uh oh, an error occurred on the audio stream: {}",
                whoopsie
            )
        };
        let music = match settings.sample_format() {
            cpal::SampleFormat::F32 => speakers.build_output_stream(
                &settings.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    heterodyne_simulator(data, channels, rate, &freq, &clock)
                },
                errorz,
                timeout
            )?,
            _ => return Err(anyhow::Error::msg("[A Rusty Theremin - Etherphone]: Uh oh, it would appear that the sample format is unsupported?")),
        };
        Ok(music)
    }
}

/// The theremin's heterodyne-effect simulator-function
/// Essentially, it generates sine-waves for audio playback at the specified frequency!
/// However, the actual 'wobble' of our rusty theremin, is generated via the user's shaky mouse-input from the GUI.
fn heterodyne_simulator<T>(
    music: &mut [T],
    channels: usize,
    rate: f32,
    freq: &Arc<AtomicUsize>,
    clock: &Arc<AtomicUsize>,
) where
    T: cpal::Sample + From<f32>,
{
    //let mut clock = 0f32;
    let mut currclock = clock.load(Ordering::Relaxed) as f32;

    let freq = freq.load(Ordering::Relaxed) as f32;

    for frame in music.chunks_mut(channels) {
        // General Inspiration from CS410P: Music, Sound, & Computers - Sine Wave Generation for le audio
        let tone = (currclock * freq * 2.0 * std::f32::consts::PI / rate).sin();

        for note in frame.iter_mut() {
            *note = T::from(tone);
        }

        currclock = (currclock + 1.0) % rate;
    }

    clock.store(currclock as usize, Ordering::Relaxed);
}
