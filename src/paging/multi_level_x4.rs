#[cfg(riscv32)]
use crate::paging::multi_level::Rv32PageTableWith;
#[cfg(riscv64)]
use crate::paging::multi_level::{Rv39PageTableWith, Rv48PageTableWith};
use crate::paging::page_table_x4::EntriesX4;
#[cfg(riscv32)]
pub type Rv32PageTableX4<'a> = Rv32PageTableWith<'a, EntriesX4>;
#[cfg(riscv64)]
pub type Rv39PageTableX4<'a> = Rv39PageTableWith<'a, EntriesX4>;
#[cfg(riscv64)]
pub type Rv48PageTableX4<'a> = Rv48PageTableWith<'a, EntriesX4>;
