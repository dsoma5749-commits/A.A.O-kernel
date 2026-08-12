#!/data/data/com.termux/files/usr/bin/bash

set -e

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

echo "======================================"
echo " A.A.O Kernel — Build & Run"
echo "======================================"

echo
echo "[1] Building kernel..."

cargo build \
    -p aao-kernel \
    --target x86_64-unknown-none

echo
echo "[2] Building UEFI bootloader..."

cargo build \
    -p aao-bootloader \
    --target x86_64-unknown-uefi

echo
echo "[3] Updating ESP..."

mkdir -p esp/EFI/BOOT

cp target/x86_64-unknown-none/debug/aao-kernel \
   esp/kernel.elf

cp target/x86_64-unknown-uefi/debug/aao-bootloader.efi \
   esp/EFI/BOOT/BOOTX64.EFI

echo
echo "[4] ESP contents:"
find esp -type f -exec ls -lh {} \;

echo
echo "[5] Starting QEMU..."
echo "--------------------------------------"

QEMU="${QEMU:-qemu-system-x86_64}"
OVMF="${OVMF:-/usr/share/ovmf/OVMF.fd}"

if ! command -v "$QEMU" >/dev/null 2>&1; then
    echo "[ERROR] qemu-system-x86_64 not found"
    exit 1
fi

if [ ! -f "$OVMF" ]; then
    echo "[ERROR] OVMF not found: $OVMF"
    exit 1
fi

exec "$QEMU" \
    -machine q35 \
    -cpu max \
    -m 512M \
    -bios "$OVMF" \
    -drive format=raw,file=fat:rw:"$ROOT/esp" \
    -serial mon:stdio \
    -display none \
    -no-reboot \
    -no-shutdown
