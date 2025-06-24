pub fn read_keyboard_events(
    display: &mut crate::os::display::Display,
) {
    use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;
}
