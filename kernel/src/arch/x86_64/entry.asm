; A.A.O Kernel - x86_64 Assembly Entry & Context Switching Helper
[bits 64]
global _asm_spin_loop
global _asm_load_gdt

section .text

_asm_spin_loop:
    hlt
    jmp _asm_spin_loop

_asm_load_gdt:
    lgdt [rdi]
    ret
