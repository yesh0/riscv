use super::frame_alloc::*;
use super::page_table::{PageTableFlags as F, *};
use super::recursive::*;
use crate::addr::*;

/// This struct is a two level page table with `Mapper` trait implemented.
pub struct TwoLevelPageTable<'a> {
    root_table: &'a mut PageTable,
    linear_offset: usize, // VA = PA + linear_offset
}

impl<'a> TwoLevelPageTable<'a> {
    pub fn new(table: &'a mut PageTable, linear_offset: usize) -> Self {
        TwoLevelPageTable {
            root_table: table,
            linear_offset,
        }
    }

    fn create_p1_if_not_exist(
        &mut self,
        p2_index: usize,
        allocator: &mut impl FrameAllocator,
    ) -> Result<&mut PageTable, MapToError> {
        if self.root_table[p2_index].is_unused() {
            let frame = allocator.alloc().ok_or(MapToError::FrameAllocationFailed)?;
            self.root_table[p2_index].set(frame.clone(), F::VALID);
            let p1_table: &mut PageTable = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p1_table.zero();
            Ok(p1_table)
        } else {
            let frame = self.root_table[p2_index].frame();
            let p1_table: &mut PageTable = unsafe { frame.as_kernel_mut(self.linear_offset) };
            Ok(p1_table)
        }
    }
}

impl<'a> Mapper for TwoLevelPageTable<'a> {
    fn map_to(
        &mut self,
        page: Page,
        frame: Frame,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocator,
    ) -> Result<MapperFlush, MapToError> {
        let p1_table = self.create_p1_if_not_exist(page.p2_index(), allocator)?;
        if !p1_table[page.p1_index()].is_unused() {
            return Err(MapToError::PageAlreadyMapped);
        }
        p1_table[page.p1_index()].set(frame, flags);
        Ok(MapperFlush::new(page))
    }

    fn unmap(&mut self, page: Page) -> Result<(Frame, MapperFlush), UnmapError> {
        if self.root_table[page.p2_index()].is_unused() {
            return Err(UnmapError::PageNotMapped);
        }
        let p1_frame = self.root_table[page.p2_index()].frame();
        let p1_table: &mut PageTable = unsafe { p1_frame.as_kernel_mut(self.linear_offset) };
        let p1_entry = &mut p1_table[page.p1_index()];
        if !p1_entry.flags().contains(F::VALID) {
            return Err(UnmapError::PageNotMapped);
        }
        let frame = p1_entry.frame();
        p1_entry.set_unused();
        Ok((frame, MapperFlush::new(page)))
    }

    fn ref_entry(&mut self, page: Page) -> Result<&mut PageTableEntry, FlagUpdateError> {
        if self.root_table[page.p2_index()].is_unused() {
            return Err(FlagUpdateError::PageNotMapped);
        }
        let p1_frame = self.root_table[page.p2_index()].frame();
        let p1_table: &mut PageTable = unsafe { p1_frame.as_kernel_mut(self.linear_offset) };
        Ok(&mut p1_table[page.p1_index()])
    }
}
