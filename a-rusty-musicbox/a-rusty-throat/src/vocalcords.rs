//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-throat Project - Vocal Cords [Audio processing - based off le a-rusty-theremin's library's etherphone.rs] Implementation File
//! This .rs file - modified from etherphone.rs of the a-rusty-theremin library - contains the audio device & stream logic for the /throat/ audio playback,
//! ...as well as the logic for the constant generation of the throat audio-stream - via the Trachea main function & heterodyne-effect simulator function!
//! Credits: Based off rodio-integration example, source: <https://github.com/lostmsu/pink-trombone/blob/master/examples/pink-trombone.rs)>
//! Dan Jang, 12/05/2023

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Stream,
};
use rand::Rng;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use pink_trombone::{NoiseSource, PinkTrombone};

pub struct Trachea {
    voice: Arc<Mutex<PinkTrombone>>,
}

/// The etherphone function - which was the original name for the throat - this function initializes the throat's audio configuration + check-processing host audio-device for playback!
impl Trachea {
    /// Etherphone constructor function!
    pub fn new(rate: u32) -> Self {
        let mut rnd = ThreadRng {};
        let throatseed = rand::thread_rng().gen();
        let voice = PinkTrombone::new(rate, &mut rnd, throatseed);

        Self {
            voice: Arc::new(Mutex::new(voice)),
        }
    }

    /// Talking (voice generation audio) & host audio-device initialization function!
    pub fn talk(&self) -> Result<Stream, anyhow::Error> {
        let host = cpal::default_host();

        let speakers = host.default_output_device().ok_or(anyhow::Error::msg("[A Rusty Throat - Vocal Cords]: Uh oh, it would appear that no compatible speakers/audio devices were detected?"))?;

        let settings = speakers.default_output_config()?;
        let _rate = settings.sample_rate().0 as f32;
        let timeout = Some(Duration::from_millis(50));
        let channels = settings.channels() as usize;

        let voice = Arc::clone(&self.voice);

        let errorz = |whoopsie| {
            eprintln!(
                "[A Rusty Throat - Vocal Cords]: Uh oh, an error occurred on the audio stream: {}",
                whoopsie
            )
        };
        let music = match settings.sample_format() {
            cpal::SampleFormat::F32 => speakers.build_output_stream(
                &settings.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut voice = voice.lock().unwrap();
                    let mut buff = [0_f32; 512];
                    let buffsize = buff.len();
                    let mut buffpos = buffsize;
                    for frame in data.chunks_mut(channels) {
                        if buffpos >= buffsize {
                            voice.synthesize(&mut buff);
                            buffpos = 0;
                        }
                        let sample = buff[buffpos];
                        for channel in frame.iter_mut() {
                            *channel = sample;// as f32;
                        }
                        buffpos += 1;
                    }
                },
                errorz,
                timeout
            )?,
            _ => return Err(anyhow::Error::msg("[A Rusty Throat - Vocal Cords]: Uh oh, it would appear that the sample format is unsupported?")),
        };
        Ok(music)
    }

    /// The getter function for the shared throat instance!
    pub fn shazam(&self) -> Arc<Mutex<PinkTrombone>> {
        Arc::clone(&self.voice)
    }
}

/// Helper functions, NoiseSource<f64> & struct ThreadRng {} for throat uniqueness, from the example provided from pink-trombone!
/// Source: https://github.com/lostmsu/pink-trombone/blob/master/examples/pink-trombone.rs)
struct ThreadRng {}

impl NoiseSource<f64> for ThreadRng {
    fn noise(&mut self) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_talk() {
        let trachea = Trachea::new(44100);
        let stream = trachea.talk();
        assert!(stream.is_ok());
    }
}
