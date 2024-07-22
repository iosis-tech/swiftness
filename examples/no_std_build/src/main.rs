#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[allow(unused_imports)]
use swiftness_air;
#[allow(unused_imports)]
use swiftness_commitment;
#[allow(unused_imports)]
use swiftness_pow;
#[allow(unused_imports)]
use swiftness_stark;