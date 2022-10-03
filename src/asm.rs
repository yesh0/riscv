//! Assembly instructions

macro_rules! instruction {
    ($(#[$attr:meta])*, $fnname:ident, $asm:expr) => (
        $(#[$attr])*
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(riscv)]
                () => core::arch::asm!($asm),

                #[cfg(not(riscv))]
                () => unimplemented!(),
            }
        }
    )
}

instruction!(
    /// `nop` instruction wrapper
    ///
    /// Generates a no-operation.  Useful to prevent delay loops from being optimized away.
    , nop, "nop");
instruction!(
    /// `EBREAK` instruction wrapper
    ///
    /// Generates a breakpoint exception.
    , ebreak, "ebreak");
instruction!(
    /// `WFI` instruction wrapper
    ///
    /// Provides a hint to the implementation that the current hart can be stalled until an interrupt might need servicing.
    /// The WFI instruction is just a hint, and a legal implementation is to implement WFI as a NOP.
    , wfi, "wfi");
instruction!(
    /// `SFENCE.VMA` instruction wrapper (all address spaces and page table levels)
    ///
    /// Synchronizes updates to in-memory memory-management data structures with current execution.
    /// Instruction execution causes implicit reads and writes to these data structures; however, these implicit references
    /// are ordinarily not ordered with respect to loads and stores in the instruction stream.
    /// Executing an `SFENCE.VMA` instruction guarantees that any stores in the instruction stream prior to the
    /// `SFENCE.VMA` are ordered before all implicit references subsequent to the `SFENCE.VMA`.
    , sfence_vma_all, "sfence.vma");

/// `SFENCE.VMA` instruction wrapper
///
/// Synchronizes updates to in-memory memory-management data structures with current execution.
/// Instruction execution causes implicit reads and writes to these data structures; however, these implicit references
/// are ordinarily not ordered with respect to loads and stores in the instruction stream.
/// Executing an `SFENCE.VMA` instruction guarantees that any stores in the instruction stream prior to the
/// `SFENCE.VMA` are ordered before all implicit references subsequent to the `SFENCE.VMA`.
#[inline]
#[allow(unused_variables)]
pub unsafe fn sfence_vma(asid: usize, addr: usize) {
    match () {
        #[cfg(riscv)]
        () => core::arch::asm!("sfence.vma {0}, {1}", in(reg) addr, in(reg) asid),

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// Blocks the program for *at least* `cycles` CPU cycles.
///
/// This is implemented in assembly so its execution time is independent of the optimization
/// level, however it is dependent on the specific architecture and core configuration.
///
/// NOTE that the delay can take much longer if interrupts are serviced during its execution
/// and the execution time may vary with other factors. This delay is mainly useful for simple
/// timer-less initialization of peripherals if and only if accurate timing is not essential. In
/// any other case please use a more accurate method to produce a delay.
#[inline]
#[allow(unused_variables)]
pub unsafe fn delay(cycles: u32) {
    match () {
        #[cfg(riscv)]
        () => {
            let real_cyc = 1 + cycles / 2;
            core::arch::asm!(
            "1:",
            "addi {0}, {0}, -1",
            "bne {0}, zero, 1b",
            inout(reg) real_cyc => _,
            options(nomem, nostack),
            )
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

mod hypervisor_extension {
    // Generating instructions for Hypervisor extension.
    // There are two kinds of instructions: rs1/rs2 type and rs1/rd type.
    // Also special register handling is required before LLVM could generate inline assembly for extended instructions.
    macro_rules! instruction_hypervisor_extension {
        (RS1_RS2, $(#[$attr:meta])*, $fnname:ident, $asm:expr) => (
            $(#[$attr])*
            #[inline]
            #[allow(unused_variables)]
            pub unsafe fn $fnname(rs1: usize, rs2: usize) {
                match () {
                    #[cfg(all(riscv))]
                    // Since LLVM does not recognize the two registers, we assume they are placed in a0 and a1, correspondingly.
                    () => core::arch::asm!($asm, in("x10") rs1, in("x11") rs2),

                    #[cfg(not(riscv))]
                    () => unimplemented!(),
                }
            }
        );
        (RS1_RD, $(#[$attr:meta])*, $fnname:ident, $asm:expr) => (
            $(#[$attr])*
            #[inline]
            #[allow(unused_variables)]
            pub unsafe fn $fnname(rs1: usize)->usize {
                match () {
                    #[cfg(all(riscv))]
                    () => {
                        let mut result : usize;
                        core::arch::asm!($asm, inlateout("x10") rs1 => result);
                        return result;
                    }

                    #[cfg(not(riscv))]
                    () => unimplemented!(),
                }
            }
        )
    }

    instruction_hypervisor_extension!(RS1_RS2,,hfence_gvma,".word 1656029299");
    instruction_hypervisor_extension!(RS1_RS2,,hfence_vvma,".word 582287475");
    instruction_hypervisor_extension!(RS1_RD,,hlv_b,".word 1610958195");
    instruction_hypervisor_extension!(RS1_RD,,hlv_bu,".word 1612006771");
    instruction_hypervisor_extension!(RS1_RD,,hlv_h,".word 1678067059");
    instruction_hypervisor_extension!(RS1_RD,,hlv_hu,".word 1679115635");
    instruction_hypervisor_extension!(RS1_RD,,hlvx_hu,".word 1681212787");
    instruction_hypervisor_extension!(RS1_RD,,hlv_w,".word 1745175923");
    instruction_hypervisor_extension!(RS1_RD,,hlvx_wu,".word 1748321651");
    instruction_hypervisor_extension!(RS1_RS2,,hsv_b,".word 1656045683");
    instruction_hypervisor_extension!(RS1_RS2,,hsv_h,".word 1723154547");
    instruction_hypervisor_extension!(RS1_RS2,,hsv_w,".word 1790263411");
    instruction_hypervisor_extension!(RS1_RD,,hlv_wu,".word 1746224499");
    instruction_hypervisor_extension!(RS1_RD,,hlv_d,".word 1812284787");
    instruction_hypervisor_extension!(RS1_RS2,,hsv_d,".word 1857372275");
}

pub use self::hypervisor_extension::*;
