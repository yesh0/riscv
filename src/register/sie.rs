//! sie register

/// sie register
#[derive(Clone, Copy, Debug)]
pub struct Sie {
    bits: usize,
}

impl Sie {
    /// Returns the contents of the register as raw bits
    #[inline(always)]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Enable
    #[inline(always)]
    pub fn usoft(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    /// Supervisor Software Interrupt Enable
    #[inline(always)]
    pub fn ssoft(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    /// User Timer Interrupt Enable
    #[inline(always)]
    pub fn utimer(&self) -> bool {
        self.bits & (1 << 4) == 1 << 4
    }

    /// Supervisor Timer Interrupt Enable
    #[inline(always)]
    pub fn stimer(&self) -> bool {
        self.bits & (1 << 5) == 1 << 5
    }

    /// User External Interrupt Enable
    #[inline(always)]
    pub fn uext(&self) -> bool {
        self.bits & (1 << 8) == 1 << 8
    }

    /// Supervisor External Interrupt Enable
    #[inline(always)]
    pub fn sext(&self) -> bool {
        self.bits & (1 << 9) == 1 << 9
    }
}

/// Reads the CSR
#[inline(always)]
pub fn read() -> Sie {
    match () {
        #[cfg(target_arch = "riscv")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0x104, x0" : "=r"(r) ::: "volatile");
            }
            Sie { bits: r }
        }
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}

/// Sets the CSR
#[cfg_attr(not(target_arch = "riscv"), allow(unused_variables))]
#[inline(always)]
unsafe fn set(bits: usize) {
    match () {
        #[cfg(target_arch = "riscv")]
        () => asm!("csrrs x0, 0x104, $0" :: "r"(bits) :: "volatile"),
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}

/// Clears the CSR
#[cfg_attr(not(target_arch = "riscv"), allow(unused_variables))]
#[inline(always)]
unsafe fn clear(bits: usize) {
    match () {
        #[cfg(target_arch = "riscv")]
        () => asm!("csrrc x0, 0x104, $0" :: "r"(bits) :: "volatile"),
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}

macro_rules! set_csr {
    ($set_field:ident, $e:expr) => {
        #[inline(always)]
        pub unsafe fn $set_field() {
            set($e);
        }
    }
}

macro_rules! clear_csr {
    ($clear_field:ident, $e:expr) => {
        #[inline(always)]
        pub unsafe fn $clear_field() {
            clear($e);
        }
    }
}

macro_rules! set_clear_csr {
    ($set_field:ident, $clear_field:ident, $e:expr) => {
        set_csr!($set_field, $e);
        clear_csr!($clear_field, $e);
    }
}

/// User Software Interrupt Enable
set_clear_csr!(set_usoft, clear_usoft, 1 << 0);
/// Supervisor Software Interrupt Enable
set_clear_csr!(set_ssoft, clear_ssoft, 1 << 1);
/// User Timer Interrupt Enable
set_clear_csr!(set_utimer, clear_utimer, 1 << 4);
/// Supervisor Timer Interrupt Enable
set_clear_csr!(set_stimer, clear_stimer, 1 << 5);
/// User External Interrupt Enable
set_clear_csr!(set_uext, clear_uext, 1 << 8);
/// Supervisor External Interrupt Enable
set_clear_csr!(set_sext, clear_sext, 1 << 9);
