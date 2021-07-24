
#![no_std]      // tell the compiler not to link the std since we run on bare metal
#![no_main]     // usually execution of a normal Rust program first starts "crt0" ("C runtime zero"), which e.g. creates a stack.
                // This then invokes the Rust runtime which is marked by the start language item. This runtime finally calls the main function
                // here is no runtime so we cant use main as entry point

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]    // dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}


// this function is called on panic
#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop {}
}