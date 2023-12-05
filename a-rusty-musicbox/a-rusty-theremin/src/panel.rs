//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-musicbox Project - Theremin GUI Implementation File (theremin.rs)
//! Dan Jang, 12/05/2023

use iced::theme;
use iced::widget::Text;
//use iced::widget::svg as lesvg;
//use iced::widget::Column as iceecolumn;
//use iced::widget::Container as iceecup;
//use iced::advanced::svg::Renderer as SVGRenderer;
//use iced::widget::svg::Handle as svgHandle;
use iced::widget::{container, svg};
use iced::{executor, Application, Command, Element, Length, Settings, Subscription, Theme};
// Correct import for Text
//use iced_native::{Event, mouse}; // Ensure you're using iced_native::Event
//use iced_futures::Event;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
//use iced_native::subscription::{self, Subscription};
//use iced_native::{Event, mouse};
//use std::hash::Hasher;

/// The public struct for the theremin panel / GUI
pub struct ThereminPanel {
    freq: Arc<AtomicUsize>,
    height: f32,
    width: f32,
}

/// A helper constant for a frequency -> note conversion table!
pub const NOTES: [(usize, &str); 14] = [
    (439, "Below A4"),
    (440, "A4"),
    (466, "A#4/B♭4"),
    (494, "B4"),
    (523, "C5"),
    (554, "C#5/D♭5"),
    (587, "D5"),
    (622, "D#5/E♭5"),
    (659, "E5"),
    (698, "F5"),
    (740, "F#5/G♭5"),
    (784, "G5"),
    (831, "G#5/A♭5"),
    (832, "Above G#5/A♭5"),
];

// impl Application for ThereminGui {
//     type Message = ();
//     type Executor = executor::Default;
//     type Flags = Arc<AtomicUsize>;
//     type Theme = Theme;

/// Helper enum for theremin GUI events
#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced::Event),
    //WindowSizeUpdates(iced::Size),
}

// /// Helper enum for implementing default Svg stuff - pt. 1
// #[derive(Default, Clone, Copy)]
// pub enum Svg {
//     Custom(fn(&Theme) -> lesvg::Appearance),
//     #[default]
//     Default,
//     //WindowSizeUpdates(iced::Size),
// }

// /// Helper impl for default Svg stuff - pt. 2
// impl lesvg::StyleSheet for Theme
// {
//     type Style = Svg;

//     fn appearance(&self, style: Self::Style) -> svg::Apearance {
//         match style {
//             Svg::Default => Default::default(),
//             Svg::Custom(appearance) => appearance(self),
//         }
//     }
// }

/// Helper function for frequency-to-note-translations!
pub fn autotune(freq: usize) -> Option<&'static str> {
    let mut nearestnote: Option<&'static str> = None;
    let mut min_diff = usize::MAX;

    for &(note_freq, note_name) in &NOTES {
        let diff = if note_freq > freq {
            note_freq - freq
        } else {
            freq - note_freq
        };

        if diff < min_diff {
            min_diff = diff;
            nearestnote = Some(note_name);
        }
    }

    //nearestnote.map(|(_, &note)| note)
    nearestnote
}

// pub fn frequency_translator(freq: usize) -> Option<&'static str> {
//     NOTES.iter().find_map(|&(lefreq, note)| {
//         if lefreq == freq {
//             Some(note)
//         } else {
//             // find the closest note

//         }
//     })
// }

/// The main implementation of the Rusty Theremin GUI panel!
/// This took most of the time, as the GUI also is vital for the 'wobbly' instrument-playing effect to properly simulate a theremin, heh.
impl Application for ThereminPanel {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<AtomicUsize>;
    type Theme = Theme;

    /// Rusty theremin GUI panel constructor
    fn new(freq: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                freq,
                height: 1000.0,
                width: 1000.0,
            },
            Command::none(),
        )
    }

    /// Rusty theremin GUI panel title
    fn title(&self) -> String {
        String::from("CS410P: Rust Programming Project - A Rusty Theremin!")
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
                //let h = 600.0; //1200.0; // 600.0;
                //let w = 720.0;
                let updatefreq = ((self.height - position.y) / self.height * 880.0) as usize;
                //let updatefreq = ((h - position.y) / h * 880.0) as usize; // Size specific for frequency-range mapping
                //let updatefreq = ((h - position.y) / h * 1760.0) as usize; // Size specific for frequency-range mapping

                self.freq.store(updatefreq, Ordering::Relaxed);
            }
            _ => {}
        }
        Command::none()
    }

    /// Rusty theremin GUI panel text - updates with the converted note from frequency if at least one note has been played!

    fn view(&self) -> Element<'_, Self::Message> {
        //let bgfile = svg::Handle::from_path("theremin-vector.svg");
        // Credits to iced author's examples, specifically: https://github.com/iced-rs/iced/tree/master/examples/svg
        let bgfile = svg::Handle::from_path(format!(
            "{}/resources/theremin_vector.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        //let bg: iced::widget::Svg<Renderer> = lesvg::Svg::new(bgfile)
        let bg = svg(bgfile)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Svg::Default);

        let txt: Text = if self.freq.load(Ordering::Relaxed) > 0 {
            let currfreq = self.freq.load(Ordering::Relaxed);
            let currnote = autotune(currfreq).unwrap_or("Out of Note Range");

            Text::new(format!(
                "[A Rusty Theremin]: Frequency: {} Hz - Nearest Note: {}",
                currfreq, currnote
            ))
            .into()
        } else {
            Text::new(format!("[A Rusty Theremin]: Hey there! You can control the pitch of the Rusty Theremin by wiggling your mouse inside le upper-half of the window! ({}x{})", self.width, self.height)).into()
        };

        let stuff = iced::widget::Column::new().push(bg).push(txt);

        container(stuff)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    // /// Rusty theremin GUI panel text - updates with current frequency if at least one note has been played!
    // fn view(&self) -> Element<'_, Self::Message> {
    //     if self.freq.load(Ordering::Relaxed) > 0 {
    //         let current_freq = self.freq.load(Ordering::Relaxed);
    //         Text::new(format!("[A Rusty Theremin]: Current Freq.: {}", current_freq)).into()
    //     } else {
    //         Text::new("[A Rusty Theremin]: Hey there! You can control the pitch of the Rusty Theremin by wiggling your mouse inside le upper-half of the window!").into()
    //     }
    // }

    /// Rusty theremin GUI panel subscription for keeping track of mouse events!
    /// This was very tricky to implement then debug for hours!
    /// Future note: iced_native may have older/putdated versions of its own modules,
    /// ...e.g. iced_native having v0.6.0 of iced_futures whilst iced has v0.7.0 - very strange
    fn subscription(&self) -> Subscription<Self::Message> {
        iced::subscription::events_with(|event, _status| Some(Message::EventOccurred(event)))
    }
    // fn subscription(&self) -> Subscription<Self::Message> {
    //     Subscription::batch(vec![
    //         iced::subscription::events_with(|event, _status| {
    //             Some(Message::EventOccurred(event))
    //     }),

    //     ])
    // }

    // fn subscription(&self) -> Subscription<Hasher, (Event, event::Status), Self::Message> {
    //     subscription::events_with(|event, _status| {
    //         Some(Message::EventOccurred(event))
    //     })
    // }
}

/// The public function for running the rusty theremin GUI window!
pub fn power_on(freq: Arc<AtomicUsize>) -> iced::Result {
    //ThereminPanel::run(Settings::default(), freq)
    ThereminPanel::run(Settings::with_flags(freq))
}
