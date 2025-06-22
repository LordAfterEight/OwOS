# OwOS
A small operating system written in Rust for funsies. It will run on any potato that has the x86_64 cpu architecture

### Booting OwOS on actual hardware
To boot this on actual hardware, copy the BOOTX64.efi file from /target/x86_64-unknown-uefi/debug/EFI/BOOT/ to /EFI/BOOT/ on a USB-stick. This directory **must** exist and **must** contain the .efi file, otherwise it won't work

### Booting OwOS in QEMU
To boot OwOS in qemu, first run the build.sh script, then run the run.sh script. That's it! :3
