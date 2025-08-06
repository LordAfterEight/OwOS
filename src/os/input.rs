use alloc::format;

use uefi::{
    Handle as UefiHandle,
    boot::{get_handle_for_protocol, open_protocol_exclusive, ScopedProtocol},
    proto::console::text::Input as Input,
};

pub fn init_input_protocol() -> ScopedProtocol<Input> {
    let handle: UefiHandle = get_handle_for_protocol::<Input>().unwrap();

    let mut stdin = open_protocol_exclusive::<Input>(handle).unwrap();

    /*
    let handle: UefiHandle = match get_handle_for_protocol::<Input>() {
        Ok(handle) => handle,
        Err(e) => {
            display.println_colored(&format!("[X] OwOS:kernel  => Failed to get Input Protocol handle: {}", e), display.colors.classic.red);
            kernel.pause(1000);
        }
    };

    let mut stdin = match open_protocol_exclusive::<Input>(handle) {
        Ok(stdin) => stdin,
        Err(e) => {
            display.println_colored(&format!("[X] OwOS:kernel  => Failed to open Input Protocol: {:?}", e), display.colors.classic.red);
            display.println_colored("[i] OwOS:kernel  => Rebooting...", display.colors.classic.yellow);
            kernel.pause(5000);
            crate::kernel::kernel::reboot();
        }
    };
    */

    return stdin;
}
