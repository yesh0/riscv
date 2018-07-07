//! sip register

/// sip register
#[derive(Clone, Copy, Debug)]
pub struct Sip {
    bits: usize,
}

impl Sip {
    /// Returns the contents of the register as raw bits
    #[inline(always)]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Pending
    #[inline(always)]
    pub fn usoft(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    /// Supervisor Software Interrupt Pending
    #[inline(always)]
    pub fn ssoft(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    /// User Timer Interrupt Pending
    #[inline(always)]
    pub fn utimer(&self) -> bool {
        self.bits & (1 << 4) == 1 << 4
    }

    /// Supervisor Timer Interrupt Pending
    #[inline(always)]
    pub fn stimer(&self) -> bool {
        self.bits & (1 << 5) == 1 << 5
    }

    /// User External Interrupt Pending
    #[inline(always)]
    pub fn uext(&self) -> bool {
        self.bits & (1 << 8) == 1 << 8
    }

    /// Supervisor External Interrupt Pending
    #[inline(always)]
    pub fn sext(&self) -> bool {
        self.bits & (1 << 9) == 1 << 9
    }
}

/// Reads the CSR
#[inline(always)]
pub fn read() -> Sip {
    match () {
        #[cfg(target_arch = "riscv")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0x144, x0" : "=r"(r) ::: "volatile");
            }
            Sip { bits: r }
        }
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}
