#![no_std]  // Don't link the Rust standard library
#![no_main]  // Disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static FUCK: &[u8] = b"Fuck you.";

#[unsafe(no_mangle)]  // Don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    // let vga_buffer = 0xb8000 as *mut u8;
    //
    // for (i, &byte) in FUCK.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    loop {}
}
