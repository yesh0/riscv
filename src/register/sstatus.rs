//! sstatus register
// TODO: Virtualization, Memory Privilege and Extension Context Fields


/// Supervisor Status Register
#[derive(Clone, Copy, Debug)]
pub struct Sstatus {
    bits: usize,
}

/// Supervisor Previous Privilege Mode
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

impl Sstatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits & (1 << 4) == 1 << 4
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits & (1 << 5) == 1 << 5
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits & (1 << 8) == (1 << 8) {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }
}


/// Reads the CSR
#[inline]
pub fn read() -> Sstatus {
    match () {
        #[cfg(target_arch = "riscv")]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0x100, x0" : "=r"(r) ::: "volatile");
            }
            Sstatus { bits: r }
        }
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}

/// Sets the CSR
#[cfg_attr(not(target_arch = "riscv"), allow(unused_variables))]
#[inline]
unsafe fn set(bits: usize) {
    match () {
        #[cfg(target_arch = "riscv")]
        () => asm!("csrrs x0, 0x100, $0" :: "r"(bits) :: "volatile"),
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}

/// Clears the CSR
#[cfg_attr(not(target_arch = "riscv"), allow(unused_variables))]
#[inline]
unsafe fn clear(bits: usize) {
    match () {
        #[cfg(target_arch = "riscv")]
        () => asm!("csrrc x0, 0x100, $0" :: "r"(bits) :: "volatile"),
        #[cfg(not(target_arch = "riscv"))]
        () => unimplemented!(),
    }
}

macro_rules! set_csr {
    ($set_field:ident, $e:expr) => {
        #[inline]
        pub unsafe fn $set_field() {
            set($e);
        }
    }
}

macro_rules! clear_csr {
    ($clear_field:ident, $e:expr) => {
        #[inline]
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

/// User Interrupt Enable
set_clear_csr!(set_uie, clear_uie, 1 << 0);
/// Supervisor Interrupt Enable
set_clear_csr!(set_sie, clear_sie, 1 << 1);
/// User Previous Interrupt Enable
set_csr!(set_upie, 1 << 4);
/// Supervisor Previous Interrupt Enable
set_csr!(set_spie, 1 << 5);
/// Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    set((spp as usize) << 8);
}
