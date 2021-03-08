use addr::*;
use core::fmt::{Debug, Error, Formatter};
use core::ops::{Index, IndexMut};

pub type Entries = [PageTableEntry; ENTRY_COUNT];

// To avoid const generic.
pub trait PTEIterableSlice {
    fn to_pte_slice<'a>(&'a self) -> &'a [PageTableEntry];
    fn to_pte_slice_mut<'a>(&'a mut self) -> &'a mut [PageTableEntry];
    fn pte_index(&self, index: usize) -> &PageTableEntry;
    fn pte_index_mut(&mut self, index: usize) -> &mut PageTableEntry;
}

impl PTEIterableSlice for Entries {
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

pub struct PageTableWith<T: PTEIterableSlice> {
    entries: T,
}

impl<T: PTEIterableSlice> PageTableWith<T> {
    /// Clears all entries.
    pub fn zero(&mut self) {
        for entry in self.entries.to_pte_slice_mut().iter_mut() {
            entry.set_unused();
        }
    }

    /// Parameter `frame` is the actual physical frame where the root page table resides,
    ///  it can be anywhere in the main memory.
    /// Denote `recursive_index` by K, then virtual address of the root page table is
    ///  (K, K+1, 0) in Sv32, and (K, K, K+1, 0) in Sv39, and (K, K, K, K+1, 0) in Sv48.
    pub fn set_recursive<F: PhysicalAddress>(
        &mut self,
        recursive_index: usize,
        frame: FrameWith<F>,
    ) {
        self[recursive_index].set(frame.clone(), EF::VALID);
        self[recursive_index + 1].set(frame.clone(), EF::VALID | EF::READABLE | EF::WRITABLE);
    }

    /// Setup identity map for the page with first level page table index.
    #[cfg(riscv32)]
    pub fn map_identity(&mut self, p2idx: usize, flags: PageTableFlags) {
        self.entries.pte_index_mut(p2idx).set(
            FrameWith::of_addr(PhysAddrSv32::new((p2idx as u64) << 22)),
            flags,
        );
    }
}

impl<T: PTEIterableSlice> Index<usize> for PageTableWith<T> {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        self.entries.pte_index(index)
    }
}

impl<T: PTEIterableSlice> IndexMut<usize> for PageTableWith<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.entries.pte_index_mut(index)
    }
}

impl<T: PTEIterableSlice> Debug for PageTableWith<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_map()
            .entries(
                self.entries
                    .to_pte_slice()
                    .iter()
                    .enumerate()
                    .filter(|p| !p.1.is_unused()),
            )
            .finish()
    }
}

pub type PageTable = PageTableWith<Entries>;

#[derive(Copy, Clone)]
pub struct PageTableEntry(usize);

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
    pub fn ppn(&self) -> usize {
        self.0 >> 10
    }
    pub fn addr<T: PhysicalAddress>(&self) -> T {
        T::new_u64((self.ppn() as u64) << 12)
    }
    pub fn frame<T: PhysicalAddress>(&self) -> FrameWith<T> {
        FrameWith::of_addr(self.addr())
    }
    pub fn set<T: PhysicalAddress>(&mut self, frame: FrameWith<T>, mut flags: PageTableFlags) {
        // U540 will raise page fault when accessing page with A=0 or D=0
        flags |= EF::ACCESSED | EF::DIRTY;
        self.0 = (frame.number() << 10) | flags.bits();
    }
    pub fn flags_mut(&mut self) -> &mut PageTableFlags {
        unsafe { &mut *(self as *mut _ as *mut PageTableFlags) }
    }
}

impl Debug for PageTableEntry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("PageTableEntry")
            .field("frame", &self.frame::<PhysAddr>())
            .field("flags", &self.flags())
            .finish()
    }
}

#[cfg(riscv64)]
pub const ENTRY_COUNT: usize = 1 << 9;
#[cfg(riscv32)]
pub const ENTRY_COUNT: usize = 1 << 10;

bitflags! {
    /// Possible flags for a page table entry.
    pub struct PageTableFlags: usize {
        const VALID =       1 << 0;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXECUTABLE =  1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
        const RESERVED1 =   1 << 8;
        const RESERVED2 =   1 << 9;
    }
}

type EF = PageTableFlags;
