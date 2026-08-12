#!/bin/bash

# build_aao.sh ফাইলে বুটলোডার কোড জেনারেট হওয়ার অংশটি ঠিক করা
python3 -c '
with open("build_aao.sh", "r") as f:
    content = f.read()

old_code = """use uefi::prelude::*;

#[entry]"""

new_code = """use uefi::prelude::*;
use core::fmt::Write;

#[global_allocator]
static ALLOCATOR: uefi::allocator::Allocator = uefi::allocator::Allocator;

#[entry]"""

if old_code in content:
    content = content.replace(old_code, new_code)
    with open("build_aao.sh", "w") as f:
        f.write(content)
    print("build_aao.sh successfully patched!")
else:
    print("Pattern not matched or already patched.")
'
