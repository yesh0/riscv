use crate::addr::*;
use crate::paging::multi_level::Rv32PageTableWith;
use crate::paging::multi_level::{Rv39PageTableWith, Rv48PageTableWith};
use crate::paging::page_table_x4::EntriesX4;
use crate::paging::recursive::MapperFlushable;
use riscv_hypervisor_extension::asm::*;

#[must_use = "Guest Page Address Table changes must be flushed or ignored."]
pub struct MapperFlushGPA(usize);

impl MapperFlushable for MapperFlushGPA {
    fn new<T: VirtualAddress>(page: PageWith<T>) -> Self {
        MapperFlushGPA(page.start_address().as_usize())
    }
    fn flush(self) {
        unsafe {
            invoke_insn_hfence_gvma(self.0, 0);
        }
    }

    fn ignore(self) {}
}

#[must_use = "Guest PageTable changes must be flushed or ignored."]
pub struct MapperFlushGPT(usize);

impl MapperFlushable for MapperFlushGPT {
    fn new<T: VirtualAddress>(page: PageWith<T>) -> Self {
        MapperFlushGPT(page.start_address().as_usize())
    }
    fn flush(self) {
        unsafe {
            invoke_insn_hfence_vvma(self.0, 0);
        }
    }

    fn ignore(self) {}
}

pub type Rv32PageTableX4<'a> = Rv32PageTableWith<'a, EntriesX4, GPAddrSv32X4, MapperFlushGPA>;
pub type Rv39PageTableX4<'a> = Rv39PageTableWith<'a, EntriesX4, GPAddrSv39X4, MapperFlushGPA>;
pub type Rv48PageTableX4<'a> = Rv48PageTableWith<'a, EntriesX4, GPAddrSv48X4, MapperFlushGPA>;
pub type Rv32PageTableGuest<'a> =
    Rv32PageTableWith<'a, super::Entries, VirtAddrSv32, MapperFlushGPT>;
pub type Rv39PageTableGuest<'a> =
    Rv39PageTableWith<'a, super::Entries, VirtAddrSv39, MapperFlushGPT>;
pub type Rv48PageTableGuest<'a> =
    Rv48PageTableWith<'a, super::Entries, VirtAddrSv48, MapperFlushGPT>;
