use log::info;
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{boot, Char16, Result, ResultExt};
use uefi_raw::table::system::SystemTable;

pub fn read_keyboard_events(input: &mut Input, display: &mut crate::os::display::Display) {
    loop {
        let mut st_ptr = uefi::table::system_table_raw().unwrap();
        let st: &mut SystemTable = unsafe { st_ptr.as_mut() };
        // Pause until a keyboard event occurs.
        let input = st.stdin;

        match input.read() {
            _ => {}
        }
    }
}
