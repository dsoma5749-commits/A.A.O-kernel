#!/bin/bash

ROOT="/data/data/com.termux/files/home/aao-kernel"

echo "======================================"
echo " A.A.O Kernel Build System"
echo " UEFI 0.29 compatible"
echo "======================================"

mkdir -p "$ROOT/bootloader/src" "$ROOT/kernel/src"

cargo fmt --all 2>/dev/null || true

echo "---- Kernel ----"
cargo build --package aao-kernel --target x86_64-unknown-none

echo ""
echo "---- UEFI Bootloader ----"
cargo build --package aao-bootloader --target x86_64-unknown-uefi
