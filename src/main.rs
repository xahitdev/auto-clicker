use enigo::{Enigo, MouseButton, MouseControllable};
use fltk::button;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use inputbot::{KeybdKey::*, *};
use std::io;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread, time,
};

fn main() {
    let app = app::App::default();
    let mut win = Window::new(300, 300, 400, 300, "Auto-Clicker");
    let mut CPSLabel = Frame::new(70, 60, 100, 40, "CPS Value");
    CPSLabel.set_label_size(20);
    let mut cpsSet = Button::new(50, 200, 150, 60, "Set CPS Value");
    let mut apply = Button::new(210, 200, 150, 60, "RUN");

    win.end();
    win.show();
    cpsSet.set_callback(move |b| {
        win.set_label("merhaba_bomboclaat");
    });

    app.run().unwrap();

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
