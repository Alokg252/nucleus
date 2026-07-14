use crate::memory::bitmap;
use crate::memory::types::{MemoryRegion, PhysicalAddress};

pub struct PhysicalMemoryManager {
    bitmap: &'static mut [u8],
    region: MemoryRegion,
    total_pages: usize,
}

impl PhysicalMemoryManager {
    pub fn new(
        bitmap: &'static mut [u8],
        region: MemoryRegion,
    ) -> Self {
        let total_pages = region.page_count();

        Self {
            bitmap,
            region,
            total_pages,
        }
    }

    pub fn alloc_frame(&mut self) -> Option<PhysicalAddress> {
        for page_index in 0..self.total_pages {
            if !bitmap::is_set(self.bitmap, page_index) {
                bitmap::set(self.bitmap, page_index);

                let address =
                    self.region.base.0 + (page_index as u64 * 4096);

                return Some(PhysicalAddress(address));
            }
        }

        None
    }

    pub fn free_frame(&mut self, address: PhysicalAddress) {
        let offset = address.0 - self.region.base.0;
        let page_index = (offset / 4096) as usize;

        bitmap::clear(self.bitmap, page_index);
    }
}