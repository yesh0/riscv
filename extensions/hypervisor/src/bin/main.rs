#![no_std]
#![feature(start)]
#![feature(llvm_asm)]
//use riscv_hypervisor_extension::csr::*;

#[start]
#[no_mangle]
pub fn my_start(argc: isize, argv: *const *const u8) -> isize {
    0
}

use core::panic::PanicInfo;
#[panic_handler]
fn onpanic(p: &PanicInfo)->!{
    loop{

    }
}