use enigo::{Enigo, MouseButton, MouseControllable};
use fltk::button;
use fltk::{app, button::Button, frame::Frame, input::Input, prelude::*, window::Window};
use inputbot::{KeybdKey::*, *};
use std::io;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    sync::Mutex,
    thread, time,
};

fn main() {
    let app = app::App::default();
    let mut win = Window::new(300, 300, 400, 300, "Auto-Clicker");
    let mut cpsSet = Button::new(50, 200, 150, 60, "Set CPS Value");
    let mut input = Input::new(250, 100, 80, 30, "Enter CPS Value...");
    let clicker_is_active = Arc::new(AtomicBool::new(false));
    let clicker_is_active_clone = Arc::clone(&clicker_is_active);
    let cps_value: u64 = 1000;

    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            if clicker_is_active_clone.load(Ordering::Relaxed) {
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(time::Duration::from_millis(cps_value));
            }
        }
    });

    fltk::dialog::alert(160, 50, "debug");

    F6Key.bind(move || {
        //callback
        clicker_is_active.store(
            !clicker_is_active.load(Ordering::Relaxed),
            Ordering::Relaxed,
        )
    });
    cpsSet.set_callback(move |b| {
        let cps_value: u64 = match input.value().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                fltk::dialog::alert(160, 50, "enter something that is a number");
                return;
            }
        };
    });

    win.end();
    win.show();

    app.run().unwrap();
    handle_input_events();

    // println!("Please enter a MS value.. E.g: 90ms = ~11cps...");
    // let mut cps_value = String::new();

    // io::stdin()
    //     .read_line(&mut cps_value)
    //     .expect("please enter a number");

    // let cps_value: u64 = match cps_value.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => {
    //         eprintln!("Please insert a NUMBER!");
    //         return;
    //     }
    // };

    // let clicker_is_active = Arc::new(AtomicBool::new(false));
    // //
    // let clicker_is_active_clone = Arc::clone(&clicker_is_active);
    // // cloning clickerisactive for the thread whick will click
    // thread::spawn(move || {
    //     let mut enigo = Enigo::new();
    //     loop {
    //         if clicker_is_active_clone.load(Ordering::Relaxed) {
    //             enigo.mouse_click(MouseButton::Left);
    //             thread::sleep(time::Duration::from_millis(cps_value));
    //         }
    //     }
    // });
    // F6Key.bind(move || {
    //     clicker_is_active.store(
    //         !clicker_is_active.load(Ordering::Relaxed),
    //         Ordering::Relaxed,
    //     );
    // });
    // handle_input_events();
}
