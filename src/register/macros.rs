macro_rules! read {
    ($register:ident, $csr_number:expr) => {
        /// Reads the CSR
        #[inline(always)]
        #[cfg(target_arch = "riscv")]
        pub fn read() -> $register {
            let r: usize;
            unsafe { asm!("csrrs $0, $1, x0" : "=r"(r) : "N"($csr_number) :: "volatile"); }
            $register { bits: r }
        }

        #[inline(always)]
        #[cfg(not(target_arch = "riscv"))]
        pub fn read() -> $register {
            unimplemented!()
        }
    };
}

macro_rules! set {
    ($register:ident, $csr_number:expr) => {
        /// Set the CSR
        #[inline(always)]
        #[cfg(target_arch = "riscv")]
        unsafe fn _set(bits: usize) {
            unsafe { asm!("csrrs x0, $1, $0" :: "r"(bits), "N"($csr_number) :: "volatile") }
        }

        #[inline(always)]
        #[cfg(not(target_arch = "riscv"))]
        unsafe fn _set(bits: usize) {
            unimplemented!()
        }
    };
}

macro_rules! clear {
    ($register:ident, $csr_number:expr) => {
        /// Clear the CSR
        #[inline(always)]
        #[cfg(target_arch = "riscv")]
        unsafe fn _clear(bits: usize) {
            unsafe { asm!("csrrc x0, $1, $0" :: "r"(bits), "N"($csr_number) :: "volatile") }
        }

        #[inline(always)]
        #[cfg(not(target_arch = "riscv"))]
        unsafe fn _clear(bits: usize) {
            unimplemented!()
        }
    };
}

macro_rules! set_csr {
    ($set_field:ident, $e:expr) => {
        #[inline(always)]
        pub unsafe fn $set_field() {
            _set($e);
        }
    }
}

macro_rules! clear_csr {
    ($clear_field:ident, $e:expr) => {
        #[inline(always)]
        pub unsafe fn $clear_field() {
            _clear($e);
        }
    }
}

macro_rules! set_clear_csr {
    ($set_field:ident, $clear_field:ident, $e:expr) => {
        set_csr!($set_field, $e);
        clear_csr!($clear_field, $e);
    }
}