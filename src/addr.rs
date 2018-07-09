#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(u32);

impl VirtAddr {
    pub fn new(addr: u32) -> VirtAddr {
        VirtAddr(addr)
    }
    pub fn as_u32(&self) -> u32 {
        self.0
    }
    pub fn p2_index(&self) -> usize {
        (self.0 as usize >> 22) & 0xfff
    }
    pub fn p1_index(&self) -> usize {
        (self.0 as usize >> 12) & 0xfff
    }
    pub fn page_offset(&self) -> usize {
        (self.0 as usize) & 0xfff
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(u32);

impl PhysAddr {
    pub fn new(addr: u32) -> PhysAddr {
        PhysAddr(addr)
    }
    pub fn as_u32(&self) -> u32 {
        self.0
    }
    pub fn p2_index(&self) -> usize {
        (self.0 as usize >> 22) & 0xfff
    }
    pub fn p1_index(&self) -> usize {
        (self.0 as usize >> 12) & 0xfff
    }
    pub fn page_offset(&self) -> usize {
        (self.0 as usize) & 0xfff
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(VirtAddr);

impl Page {
    pub fn of_addr(addr: VirtAddr) -> Self {
        Page(addr)
    }
    pub fn start_address(&self) -> VirtAddr {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(PhysAddr);

impl Frame {
    pub fn of_addr(addr: PhysAddr) -> Self {
        Frame(addr)
    }
    pub fn start_address(&self) -> PhysAddr {
        self.0.clone()
    }
}