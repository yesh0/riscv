//! time register

/// Reads the CSR
#[inline(always)]
pub fn read() -> usize {
    match () {
        #[cfg(target_arch = "riscv")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0xC01, x0" : "=r"(r) ::: "volatile");
            }
            r
        }
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}
