//! CS410P: Rust Programming, Course Project - Fall 2023
//! a-rusty-musicbox Project - Main Binary Implementation File
//! Dan Jang, 12/05/2023

use a_rusty_theremin::*;
use a_rusty_throat::*;
use prompted::input;

/// The main project function, as a wrapper of our two instruments!
fn main() {
    println!("Welcome to A Rusty Musicbox!");

    println!("\nA Rusty Musicbox is a wrapper library containing two very unique, musical instruments based on freeform, user-input.");
    println!("\n-----\nThe first instrument is a normal theremin, where your mouse cursor movement inside the GUI instrument window controls the pitch and volume of the theremin.");
    println!("\n-----\nThe second instrument is a vocal theremin - based on the 'pink-trombone' (by Neil Thapen, 2017) vocal speech synthesizer...\n");
    println!("...where the user plays the instrument in the same way, via the mouse-cursor, but the sounds are generated through the 'pink-trombone' synthesizer!");
    println!("-----\n\n");
    println!("----------\n");
    println!("For each instrument, there will be statistics displayed & live normalization of the mouse-cursor input (based on resizing the GUI window) - both updated dynamically.");
    println!("In instrument 1, the nearest note corresponding to the current Theremin pitch is displayed, e.g. A4 (440 Hz) - as well as the specific frequency!");

    println!("Specifically, the live normalization of the mouse-cursor input is essential in preserving the same/relative control schema in playing each instrument.");
    println!("At any time, if the GUI window for either instrument is resized, the live normalization dynamically preserves the same degree of instrument playing control, which is pretty neat.");

    println!("This is a rather silly project, but I hope you enjoy it!");
    println!("-----\n!!!\nNote: Instrument 1 is preset to 127 Hz when it launches, so as to avoid an uncomfortable blast of screeching sound - but just in case, make sure to adjust your volume now before proceeding with either instrument!\n!!!\n-----\n");

    println!("Due to the messy nature of audio/device logic, this project will only launch one instrument per execution, to avoid EventLoop related issues.");
    println!("However, please feel free to re-launch to launch either of the instruments!\n");

    println!("Here are the following two instruments + exit option numbers:\n");
    println!("1. A Rusty Theremin - The normal theremin, with live nearest musical note matching & raw frequency (Hz) stats + dynamic GUI window input-normalization (since input is mouse-cursor position based)");
    println!("2. A Rusty Throat - The vocal, speech synthesizer-based theremin instrument, with live statistics of 'pink-trombone'-specific parameters, e.g. vibrato, tension, target notes, etc.");

    println!("-----\nTo exit the project, please enter 3.");
    let optionnum = 3;

    let userinput = input!("Please enter your choice, as a whole number from 1 to {} (e.g., type the number 1, then press enter, to explore the first instrument): ", optionnum);
    match userinput.parse::<isize>() {
        Ok(num) => {
            if (1..=3).contains(&num) {
                println!("You chose option number {}!", num);
                if num == 1 {
                    println!("\nNow loading... 1. A Rusty Theremin - The normal theremin!\n");
                    let _ = musictime();
                } else if num == 2 {
                    println!("\nNow loading... 2. A Rusty Throat - The vocal, speech synthesizer-based theremin!\n");
                    let _ = sing();
                } else {
                    println!("\nOkey dokey, project is now exiting!\n");
                }
            }
        }
        Err(nonnum) => println!(
            "Sorry, {} is not a valid number between 1 & {} - please try again!",
            nonnum, optionnum
        ),
    };

    println!("Thank you for (hopefully) having fun with my silly project, A Rusty Musicbox!");
    println!("Have a lovely day, afternoon, evening, and/or night!");
}
