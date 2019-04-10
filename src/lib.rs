#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn rusty_add(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

#[panic_handler]
fn internal_panic(_info: &PanicInfo) -> ! {
    loop {
    }
}
