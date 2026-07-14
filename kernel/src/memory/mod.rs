pub mod pmm;
pub mod types;
pub mod bitmap;

use crate::memory::types::{
    MemoryRegion,
    PhysicalAddress,
};

const MAX_MEMORY_REGIONS: usize = 128;

static mut MEMORY_REGIONS: [MemoryRegion; MAX_MEMORY_REGIONS] =
    [MemoryRegion {
        base: PhysicalAddress(0),
        length: 0,
    }; MAX_MEMORY_REGIONS];

static mut MEMORY_REGION_COUNT: usize = 0;