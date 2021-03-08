pub trait Address: core::fmt::Debug + Copy+ Clone+ PartialEq+ Eq+ PartialOrd+ Ord {
    fn new(addr: usize)->Self;
    fn page_number(&self)->usize;
    fn page_offset(&self)->usize;
    fn to_4k_aligned(&self)->Self;
    fn as_usize(&self)->usize;
}

pub trait VirtualAddress: Address {
    unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T; 
}

pub trait AddressX32: Address{
    fn new_u32(addr: u32) -> Self;
    fn as_u32(&self)->u32;
}
pub trait AddressX64: Address{
    fn new_u64(addr: u64) -> Self;
    fn as_u64(&self)->u64;
}

pub trait PhysicalAddress: AddressX64 {

}

pub trait AddressL3: Address{
    fn p3_index(&self)->usize;
    fn p2_index(&self)->usize;
    fn p1_index(&self)->usize;
    fn from_page_table_indices(
        p3_index: usize,
        p2_index: usize,
        p1_index: usize,
        offset: usize,
    )->Self;
}

pub trait AddressL4: Address{
    fn p4_index(&self)->usize;
    fn p3_index(&self)->usize;
    fn p2_index(&self)->usize;
    fn p1_index(&self)->usize;
    fn from_page_table_indices(
        p4_index: usize,
        p3_index: usize,
        p2_index: usize,
        p1_index: usize,
        offset: usize,
    )->Self;
}


pub trait AddressL2: Address{
    fn p2_index(&self)->usize;
    fn p1_index(&self)->usize;
    fn from_page_table_indices(
        p2_index: usize,
        p1_index: usize,
        offset: usize,
    )->Self;
}
pub mod sv32;
pub mod sv39;
pub mod sv48;
pub mod gpax4;
pub mod page;

pub use self::sv32::*;
pub use self::sv39::*;
pub use self::sv48::*;
pub use self::gpax4::*;
pub use self::page::*;



//#[cfg(target= "riscv64")]
pub type VirtAddr = VirtAddrSv48;
pub type PhysAddr = PhysAddrSv48;


pub type Page = PageWith<VirtAddr>;
pub type Frame = FrameWith<PhysAddr>;