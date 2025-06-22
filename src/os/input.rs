use log::info;
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{boot, Char16, Result, ResultExt};

pub fn read_keyboard_events(input: &mut Input, display: &mut crate::os::display::Display) {
    loop {
        // Pause until a keyboard event occurs.
        let mut events = unsafe { [input.wait_for_key_event().unwrap()] };
        boot::wait_for_event(&mut events).discard_errdata().unwrap();

        match input.read_key().unwrap() {
            // Example of handling a printable key: print a message when
            // the 'u' key is pressed.
            Some(Key::Printable(key)) if key == 'u' => {
                info!("the 'u' key was pressed");
            }

            // Example of handling a special key: exit the loop when the
            // escape key is pressed.
            Some(Key::Special(ScanCode::ESCAPE)) => {
                break;
            }
            _ => {}
        }
    }
}
