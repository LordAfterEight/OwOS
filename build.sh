#!/bin/bash


export CARGO_MANIFEST_DIR=$(pwd)

git pull

cargo build -Zbuild-std=core,alloc --no-default-features

mkdir -p target/x86_64-unknown-uefi/debug/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/OwOS.efi target/x86_64-unknown-uefi/debug/EFI/BOOT/BOOTX64.EFI
