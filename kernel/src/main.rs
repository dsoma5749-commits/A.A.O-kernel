#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod arch;
mod isolation;

use isolation::{
    Capability, CapabilityId, CapabilityType, EndpointId, IpcEndpoint, IpcMessage, Ring3Domain,
    ServiceId, UserEntry, UserService,
};

const STACK_SIZE: usize = 16 * 1024;

#[repr(align(16))]
#[allow(dead_code)]
pub struct KernelStack([u8; STACK_SIZE]);

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".bss")]
pub static mut KERNEL_STACK: KernelStack = KernelStack([0; STACK_SIZE]);

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".bss")]
pub static mut AAO_BSS_TEST: [u8; 4096] = [0; 4096];

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    arch::x86_64::initialize();

    isolation_bootstrap();

    loop {
        core::hint::spin_loop();
    }
}

fn isolation_bootstrap() {
    /*
     * M1:
     * Kernel owns the capability authority.
     * User services receive explicit capabilities.
     * This is the first security boundary.
     */

    let ipc_capability = Capability::new(
        CapabilityId::new(1),
        CapabilityType::IpcEndpoint,
        Capability::READ | Capability::WRITE,
    );

    let ipc_token = isolation::capability::IpcCapability::new(ipc_capability);

    let endpoint = IpcEndpoint::new(EndpointId::new(1));

    let mut message = IpcMessage::empty();

    let _ = message.push(0xAA4F);
    let _ = message.push(1);

    let _ = endpoint.send(&ipc_token, &message);

    let service_capability = isolation::service::service_capability(2);

    let mut filesystem = UserService::new(ServiceId::new(1), service_capability);

    let _ = filesystem.start();

    /*
     * Ring-3 domain instantiation test
     */
    let user_entry = UserEntry::new(0x0040_0000, 0x0080_0000);

    let _user_domain = match Ring3Domain::new(user_entry) {
        Ok(domain) => domain,
        Err(_) => return,
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kernel_main()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
