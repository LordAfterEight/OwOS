#!/bin/bash

export CARGO_MANIFEST_DIR=$(pwd)

cargo build -Zbuild-std=core,alloc --no-default-features

mkdir -p target/x86_64-unknown-uefi/debug/EFI/BOOT
cp /home/elias/rust/owos/target/x86_64-unknown-uefi/debug/owos.efi /home/elias/rust/owos/target/x86_64-unknown-uefi/debug/EFI/BOOT/BOOTX64.EFI

qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:/home/elias/rust/owos/target/x86_64-unknown-uefi/debug/
