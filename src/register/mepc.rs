//! mepc register

/// Reads the CSR
#[inline(always)]
pub fn read() -> u32 {
    match () {
        #[cfg(target_arch = "riscv32")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0x341, x0" : "=r"(r) ::: "volatile");
            }
            r as u32
        },
        #[cfg(not(target_arch = "riscv32"))]
        () => unimplemented!(),
    }
}
