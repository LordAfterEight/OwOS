#!/bin/bash


export CARGO_MANIFEST_DIR=$(pwd)

cargo build -Zbuild-std=core,alloc --no-default-features --release

mkdir -p target/x86_64-unknown-uefi/release/EFI/BOOT
cp target/x86_64-unknown-uefi/release/OwOS.efi target/x86_64-unknown-uefi/release/EFI/BOOT/BOOTX64.EFI
