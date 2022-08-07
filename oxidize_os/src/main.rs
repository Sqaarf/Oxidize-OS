
#![no_std] // Disabling the standard library
#![no_main] // Disabling the Rust-level entry point

// Handling panic messages
use core::panic::PanicInfo;

// Diverging function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entrypoint (Start of the program)
#[no_mangle] // Disabling name mangling, allowing the name to be used by the linker
pub extern "C" fn _start() -> ! { // pub extern "C" to allow calling from C
    loop{}
}

// To build the project :
// rustup target add thumbv7em-none-eabihf 
// `cargo build --target thumbv7em-none-eabihf`