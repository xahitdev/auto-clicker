use enigo::{Enigo, MouseButton, MouseControllable};
use inputbot::{KeybdKey::*, *};
use std::io;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread, time,
};

fn main() {
    println!("Please enter a MS value.. E.g: 90ms = ~11cps...");

    let mut cps_value = String::new();

    io::stdin()
        .read_line(&mut cps_value)
        .expect("please enter a number");

    let cps_value: u64 = match cps_value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please insert a NUMBER!");
            return;
        }
    };

    let clicker_is_active = Arc::new(AtomicBool::new(false));
    //
    let clicker_is_active_clone = Arc::clone(&clicker_is_active);
    // cloning clickerisactive for the thread whick will click
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            if clicker_is_active_clone.load(Ordering::Relaxed) {
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(time::Duration::from_millis(cps_value));
            }
        }
    });
    F6Key.bind(move || {
        clicker_is_active.store(
            !clicker_is_active.load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
    });
    handle_input_events();
}
