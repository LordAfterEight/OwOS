#!/bin/bash
qemu-system-x86_64 -bios /home/elias/rust/owos/OVMF_X64.fd -drive format=raw,file=fat:rw:/home/elias/rust/owos/target/x86_64-unknown-uefi/release/
