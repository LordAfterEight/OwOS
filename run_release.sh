#!/bin/bash
qemu-system-x86_64 -bios OVMF_X64.fd -drive format=raw,file=fat:rw:target/x86_64-unknown-uefi/release/
