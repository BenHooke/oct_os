#![no_std]  // Don't link the Rust standard library
#![no_main]  // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[ failed ]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[unsafe(no_mangle)]  // Don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Test {}", "print.");

    #[cfg(test)]
    test_main();

    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    let mut grammar = "tests";
    if tests.len() <= 1 {
        grammar = "test";
    } 

    serial_println!("Running {} {}", tests.len(), grammar);
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("Trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ ok ]");
}

#[test_case]
fn trivial_assertion_2() {
    serial_print!("Trivial assertion 2... ");
    assert_eq!(2, 2);
    serial_println!("[ ok ]");
}
