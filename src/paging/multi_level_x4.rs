use crate::paging::multi_level::Rv32PageTableWith;
use crate::paging::multi_level::{Rv39PageTableWith, Rv48PageTableWith};
use crate::paging::page_table_x4::EntriesX4;
use crate::addr::gpax4::*;
pub type Rv32PageTableX4<'a> = Rv32PageTableWith<'a, EntriesX4, GPAddrSv32X4>;
pub type Rv39PageTableX4<'a> = Rv39PageTableWith<'a, EntriesX4, GPAddrSv39X4>;
pub type Rv48PageTableX4<'a> = Rv48PageTableWith<'a, EntriesX4, GPAddrSv48X4>;
