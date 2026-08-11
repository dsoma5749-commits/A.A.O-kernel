#![allow(dead_code)]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
#[allow(unused_imports)]
pub use self::x86_64::{init_paging, initialize, Architecture, UserAddressSpace};
