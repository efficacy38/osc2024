#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod bsp;
mod cpu;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!()
}

unsafe fn kernel_init() -> ! {
    panic!()
}
