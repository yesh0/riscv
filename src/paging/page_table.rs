use addr::*;
use core::ops::{Index, IndexMut};

pub struct PageTable {
    entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    /// Clears all entries.
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

pub struct PageTableEntry(u32);

impl PageTableEntry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }
    pub fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.0)
    }
    pub fn addr(&self) -> PhysAddr {
        PhysAddr::new((self.0 << 2) & 0xfffff000)
    }
    pub fn frame(&self) -> Frame {
        Frame::of_addr(self.addr())
    }
    pub fn set(&mut self, frame: Frame, flags: PageTableFlags) {
        self.0 = (frame.number() << 10) as u32 | flags.bits();
    }
}

const ENTRY_COUNT: usize = 1 << 10;

bitflags! {
    /// Possible flags for a page table entry.
    pub struct PageTableFlags: u32 {
        const VALID =       1 << 0;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXCUTABLE =   1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
        const RESERVED1 =   1 << 8;
        const RESERVED2 =   1 << 9;
    }
}