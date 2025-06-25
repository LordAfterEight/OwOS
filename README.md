# OwOS
A micro-OS written in Rust for funsies. It will run on any potato that has the x86_64 cpu architecture

### Booting OwOS on actual hardware
1. **Newest version:** Run the ```build_release.sh``` script and copy the ```BOOTX64.EFI``` file from ```target/x86_64-unknown-uefi/release/EFI/BOOT/``` into the ```EFI/BOOT/``` directory on a USB-stick. If that directory doesn't exist, create it.
2. **Latest release:** Download the ```BOOTX64.EFI``` file and copy it into the ```EFI/BOOT/``` directory on a USB-stick. If that directory doesn't exist, create it.

### Booting OwOS in QEMU
To boot OwOS in qemu, run the build_release.sh script, then run the run_release.sh script. That's it ! :3
