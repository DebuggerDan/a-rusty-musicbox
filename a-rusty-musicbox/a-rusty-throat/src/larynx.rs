//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-throat Project - Larynx [GUI - based off le a-rusty-throat library's panel.rs] Implementation File
//! Dan Jang, 12/05/2023

// use iced::theme;
use iced::widget::Text;
use pink_trombone::PinkTrombone;

use iced::widget::{container, image};
use iced::{executor, Application, Command, Element, Length, Settings, Subscription, Theme};

//use std::sync::atomic::{Ordering};
use std::sync::{Arc, Mutex};

/// The public struct for the throat panel / GUI
pub struct ThroatPanel {
    voice: Arc<Mutex<PinkTrombone>>,
    //freq: Arc<AtomicUsize>,
    height: f32,
    width: f32,
    lastx: f32,
    lasty: f32,
}

/// Helper enum for throat GUI events
#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced::Event),
    //WindowSizeUpdates(iced::Size),
}

/// The main implementation of the Rusty Throat GUI panel!
impl Application for ThroatPanel {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<Mutex<PinkTrombone>>; //Arc<AtomicUsize>;
    type Theme = Theme;

    /// Rusty throat GUI panel constructor
    fn new(voice: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                //freq,
                voice,
                height: 1000.0,
                width: 1000.0,
                lastx: 0.0,
                lasty: 0.0,
            },
            Command::none(),
        )
    }

    /// Rusty throat GUI panel title
    fn title(&self) -> String {
        String::from("CS410P: Rust Programming Project - A Rusty Throat! [Vocal Theremin Horror]")
    }

    fn update(&mut self, pitch: Self::Message) -> Command<Self::Message> {
        match pitch {
            Message::EventOccurred(iced::Event::Window(iced::window::Event::Resized {
                width,
                height,
            })) => {
                self.height = height as f32;
                self.width = width as f32;
            }
            Message::EventOccurred(iced::Event::Mouse(iced::mouse::Event::CursorMoved {
                position,
            })) => {
                //let updatefreq = ((self.height - position.y) / self.height * 880.0) as usize;
                self.lastx = position.x;
                self.lasty = position.y;
                let mut voice = self.voice.lock().unwrap();

                voice.set_musical_note((position.y / self.height) * 24.0);
                let vibratotest = position.x / self.width;

                let _mouthopen_x = position.x / self.width;
                let _mouthopen_y = position.y / self.height;

                if (position.x > 0.1) && (position.y > 0.1) {
                    voice.set_velum_open(true);
                } else {
                    voice.set_velum_open(false);
                }

                if vibratotest > 0.5 {
                    voice.set_vibrato_wobble(true);
                    voice.set_vibrato_frequency(vibratotest);
                } else {
                    voice.set_vibrato_wobble(false);
                }

                //voice.set_vibrato_wobble((position.x / self.width) as f32);
                voice.set_target_tenseness(position.y / self.height);
                //self.freq.store(updatefreq, Ordering::Relaxed);
            }
            _ => {}
        }
        Command::none()
    }

    /// Rusty throat GUI panel text - updates with the converted note from frequency if at least one note has been played!
    fn view(&self) -> Element<'_, Self::Message> {
        // Credits to iced author's examples, specifically: https://github.com/iced-rs/iced/tree/master/examples/pokedex [for png/image]
        // Photo Credits & Source: https://experiments.withgoogle.com/pink-trombone
        let bgfile = image::Handle::from_path(format!(
            "{}/resources/new_pinktrombone.png",
            env!("CARGO_MANIFEST_DIR")
        ));

        let bg = image(bgfile).width(Length::Fill).height(Length::Fill);
        //.style(theme::Image::Default);

        let txt: Text = if (self.lastx > 0.1) && (self.lasty > 0.1) {
            //let currfreq = self.freq.load(Ordering::Relaxed);
            //let currnote = autotune(currfreq).unwrap_or("Out of Note Range");
            let currnote = (self.lasty / self.height) * 24.0_f32;
            let mouthopen_x = self.lastx / self.width;
            let mouthopen_y = self.lasty / self.height;

            let mut vibratobool: &str = "No";

            if mouthopen_x > 0.5 {
                vibratobool = "Yes";
            }

            Text::new(format!(
                "[A Rusty Throat]: Musical Note (0.0~24.0): {}, Vibrato Wobble (yes/no): {}, & Target Tenseness (f32): {}",
                currnote, vibratobool, mouthopen_y
            ))
        } else {
            Text::new(format!("[A Rusty Throat]: Hey there! You can control the pitch of the Rusty Throat by wiggling your mouse inside le upper-half of the window! ({}x{})", self.width, self.height))
        };

        let stuff = iced::widget::Column::new().push(bg).push(txt);

        container(stuff)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    /// Rusty throat GUI panel subscription for keeping track of mouse events!
    /// This was very tricky to implement then debug for hours!
    /// Future note: iced_native may have older/putdated versions of its own modules,
    /// ...e.g. iced_native having v0.6.0 of iced_futures whilst iced has v0.7.0 - very strange
    fn subscription(&self) -> Subscription<Self::Message> {
        iced::subscription::events_with(|event, _status| Some(Message::EventOccurred(event)))
    }
}

/// The public function for running the rusty throat GUI window!
pub fn warm_up(voice: Arc<Mutex<PinkTrombone>>) -> iced::Result {
    //ThroatPanel::run(Settings::default(), freq)
    ThroatPanel::run(Settings::with_flags(voice))
}
