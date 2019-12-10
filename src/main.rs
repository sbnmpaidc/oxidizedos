#![no_std]
#![no_main]
#![feature(alloc_error_handler)]


mod machine;
mod u8250;
mod config;
mod heap;

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec};

use core::fmt::Write;
use core::panic::PanicInfo;

use u8250::U8250;
use config::mb_info;
use heap::{Heap, LockedHeap, Block};


static HELLO: &[u8] = b"Off to the races!\n";


#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::new();

pub fn main() {}

#[no_mangle]
pub extern "C" fn _start(mb_config: &mb_info) -> ! {
    let mut uart = U8250 {};
    let hi = "Hello there!\n";
    uart.write_string(hi);
    write!(uart, "The numbers are {} and {}, {}\n", 42, 1.0 / 3.0, hi).unwrap();
    println!("ooooweee, we're using println, {} {} {}", 42, 1.0 / 3.0, hi);
    mb_config.print();
    //println!("mb config at 0x{:x}", mb_config as *const u32);
    for (i, &byte) in HELLO.iter().enumerate() {
        uart.put(byte as u8);
    }
    let heap_val = Box::new(41);
    println!("value on heap {}", heap_val);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_panic(layout: alloc::alloc::Layout) -> ! {
    panic!("Failure in alloc");
}
