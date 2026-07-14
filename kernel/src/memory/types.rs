#[derive(Clone, Copy, Debug)]
pub struct PhysicalAddress(pub u64);

pub struct PhysicalMemoryManager {
    bitmap: &'static mut [u8],
    total_pages: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryRegion {
    pub base: PhysicalAddress,
    pub length: u64,
}

impl MemoryRegion {
    pub fn page_count(&self) -> usize {
        (self.length / 4096) as usize
    }
}