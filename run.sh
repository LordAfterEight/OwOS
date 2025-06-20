#!/bin/bash
qemu-system-x86_64 -bios /home/elias/rust/owos/RELEASEX64_OVMF.fd -drive format=raw,file=fat:rw:/home/elias/rust/owos/target/x86_64-unknown-uefi/debug/
