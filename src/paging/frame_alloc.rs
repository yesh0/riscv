//! Traits for abstracting away frame allocation and deallocation.

use addr::{FrameWith, PhysicalAddress};
/// A trait for types that can allocate a frame of memory.
pub trait FrameAllocator {
    /// Allocate a frame of the appropriate size and return it if possible.
    fn alloc<P: PhysicalAddress>(&mut self) -> Option<FrameWith<P>>;
}

/// A trait for types that can deallocate a frame of memory.
pub trait FrameDeallocator {
    /// Deallocate the given frame of memory.
    fn dealloc<P: PhysicalAddress>(&mut self, frame: FrameWith<P>);
}
