//! Low level access to RISC-V processors
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.31 and up. It *might*
//! compile with older versions but that may change in any new patch release.
//! Note that `riscv64imac-unknown-none-elf` and `riscv64gc-unknown-none-elf` targets
//! are not supported on stable yet.
//!
//! # Features
//!
//! This crate provides:
//!
//! - Access to core registers like `mstatus` or `mcause`.
//! - Interrupt manipulation mechanisms.
//! - Wrappers around assembly instructions like `WFI`.

#![no_std]
#![deny(warnings)]
#![cfg_attr(feature = "inline-asm", feature(llvm_asm))]
#![feature(type_alias_impl_trait)]
extern crate bare_metal;
#[macro_use]
extern crate bitflags;
extern crate bit_field;
#[cfg(feature = "hypervisor")]
extern crate riscv_hypervisor_extension;

pub mod addr;
pub mod asm;
pub mod interrupt;
pub mod paging;
pub mod register;
