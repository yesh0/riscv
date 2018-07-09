//! Assembly instructions

macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline(always)]
        pub fn $fnname() {
            match () {
                #[cfg(target_arch = "riscv")]
                () => unsafe {
                    asm!($asm :::: "volatile");
                },
                #[cfg(not(target_arch = "riscv"))]
                () => {}
            }
        }
    )
}


/// Priviledged ISA Instructions
instruction!(ecall, "ecall");
instruction!(ebreak, "ebreak");
instruction!(uret, "uret");
instruction!(sret, "sret");
instruction!(mret, "mret");
instruction!(wfi, "wfi");

use addr::VirtAddr;

#[inline(always)]
pub fn sfence_vma_all() {
    unsafe{ asm!("sfence.vma" :::: "volatile"); }
}

#[inline(always)]
pub fn sfence_vma(asid: usize, addr: VirtAddr) {
    unsafe{ asm!("sfence.vma $0, $1" :: "r"(asid), "r"(addr.as_usize()) :: "volatile"); }
}