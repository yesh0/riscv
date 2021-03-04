/// This file is for Hypervisor-related x4 page tables, including Sv32x4, Sv39x4 and Sv48x4.
/// In fact, these x4 page tables are Phys-to-Phys page tables from GPAs to real PAs.
use super::page_table::{PTEIterableSlice, PageTableEntry, PageTableWith, ENTRY_COUNT};

// The root page table is 4 times larger.
pub const X4_ENTRY_COUNT: usize = ENTRY_COUNT << 2;

pub type EntriesX4 = [PageTableEntry; X4_ENTRY_COUNT];

impl PTEIterableSlice for EntriesX4 {
    fn to_pte_slice(&self) -> &[PageTableEntry] {
        self
    }
    fn to_pte_slice_mut(&mut self) -> &mut [PageTableEntry] {
        self
    }
    fn pte_index(&self, index: usize) -> &PageTableEntry {
        &self[index]
    }
    fn pte_index_mut(&mut self, index: usize) -> &mut PageTableEntry {
        &mut self[index]
    }
}
pub type PageTableX4 = PageTableWith<EntriesX4>;
