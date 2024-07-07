use enigo::{Enigo, MouseButton, MouseControllable};
use fltk::{
    app,
    button::Button,
    enums::{Event, Key},
    input::Input,
    prelude::*,
    window::Window,
};
use inputbot::{KeybdKey::*, *};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 200, "Auto Clicker");

    let mut input = Input::new(160, 50, 80, 30, "Enter MS value:");
    let mut btn = Button::new(160, 100, 80, 40, "Set Value");

    let clicker_is_active = Arc::new(AtomicBool::new(false));
    let cps_value = Arc::new(std::sync::Mutex::new(90u64)); // Default value

    let clicker_is_active_clone = Arc::clone(&clicker_is_active);
    let cps_value_clone = Arc::clone(&cps_value);

    btn.set_callback({
        let cps_value_clone = Arc::clone(&cps_value);
        move |_| {
            let input_value: u64 = match input.value().trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    fltk::dialog::alert(160, 50, "Please insert a NUMBER!");
                    return;
                }
            };

            let mut cps_value_lock = cps_value_clone.lock().unwrap();
            *cps_value_lock = input_value;
            println!("Set CPS Value to {} ms", input_value);
        }
    });

    // Thread to handle the clicking
    let clicker_is_active_clone = Arc::clone(&clicker_is_active);
    let cps_value_clone = Arc::clone(&cps_value);
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            if clicker_is_active_clone.load(Ordering::Relaxed) {
                let cps_value_lock = cps_value_clone.lock().unwrap();
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(time::Duration::from_millis(*cps_value_lock));
            } else {
                thread::sleep(time::Duration::from_millis(100));
            }
        }
    });
    wind.handle({
        let clicker_is_active = Arc::clone(&clicker_is_active);
    });
    // F6 key event to toggle the clicker
    // wind.handle({
    //     let clicker_is_active = Arc::clone(&clicker_is_active);
    //     move |_, ev| match ev {
    //         Event::KeyDown => {
    //             if app::event_key() == Key::F6 {
    //                 let new_state = !clicker_is_active.load(Ordering::Relaxed);
    //                 clicker_is_active.store(new_state, Ordering::Relaxed);
    //                 println!(
    //                     "Auto-clicker is now {}",
    //                     if new_state { "ON" } else { "OFF" }
    //                 );
    //             }
    //             true
    //         }
    //         _ => false,
    //     }
    // });
    println!("evet abi");

    wind.end();
    wind.show();

    app.run().unwrap();
}
