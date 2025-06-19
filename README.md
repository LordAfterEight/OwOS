# OwOS
A small operating system written in Rust for funsies. It will run on any potato that has the x86_64 cpu architecture

### Booting OwOS on actual hardware
To boot this on actual hardware, write the .bin file to a USB-stick using a program like balena etcher, insert the USB-stick into your computer, restart, enter the bios and select the USB-stick as the first boot option. Simply exit the bios and the pc will restart and boot into OwOS! :3

This build is known to randomly panic. In case that happens, simply hit the restart button of your pc and then it should work. If not, rinse and repeat lol

### Booting OwOS in QEMU
To boot OwOS in qemu, download the .bin file. Put it whereever you want and navigate to it in your terminal. Then enter the following command:

```bash
qemu-system-x86_64 --drive format=raw,file=bootimage-owos-[version].bin
```

And it should fire right up! :3
