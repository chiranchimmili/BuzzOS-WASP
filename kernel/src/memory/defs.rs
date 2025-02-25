use bitflags::bitflags;

use crate::structures::static_linked_list::StaticLinkedListNode;

/// Macros

/// Perform page rounding up, to the next page boundary
#[macro_export]
macro_rules! ROUND_UP {
    ($val:expr, $align:expr) => {
        ($val + $align - 1) & !($align - 1)
    };
}

/// Perform page rounding down, to the previous page boundary
#[macro_export]
macro_rules! ROUND_DOWN {
    ($val:expr, $align:expr) => {
        $val & !($align - 1)
    };
}

/// Convert memory address from virtual (above KERNEL_BASE) to physical (below KERNEL_BASE)
#[macro_export]
macro_rules! V2P {
    ($n:expr) => {
        ($n) - KERNEL_BASE
    };
}

/// Convert memory address from physical (below KERNEL_BASE) to virtual (above KERNEL_BASE)
#[macro_export]
macro_rules! P2V {
    ($n:expr) => {
        ($n) + KERNEL_BASE
    };
}

#[macro_export]
macro_rules! PAGE_TABLE_INDEX {
    ($n:expr) => {
        ($n >> PAGE_TABLE_SHIFT) & 0x3FF
    };
}

#[macro_export]
macro_rules! PAGE_DIR_INDEX {
    ($n:expr) => {
        ($n >> PAGE_DIR_SHIFT) & 0x3FF
    };
}

/// GDT Definitions
pub const N_DESCRIPTORS: usize = 6;

pub const GDT_FLAG_L: u8 = 0x2;
pub const GDT_FLAG_DB: u8 = 0x4;
pub const GDT_FLAG_G: u8 = 0x8;
pub const GDT_TYPE_A: u8 = 0x1;
pub const GDT_TYPE_RW: u8 = 0x2;
pub const GDT_TYPE_DC: u8 = 0x4;
pub const GDT_TYPE_E: u8 = 0x8;
pub const GDT_RING0: u8 = 0x00;
pub const GDT_RING1: u8 = 0x20;
pub const GDT_RING2: u8 = 0x40;
pub const GDT_RING3: u8 = 0x60;
pub const GDT_TYPE_S: u8 = 0x10;
pub const GDT_TYPE_P: u8 = 0x80;

/// VM Definitions
pub const PAGE_SIZE: usize = 4096;

pub const EXTENDED_MEMORY: usize = 0x100000;
pub const DEVICE_SPACE: usize = 0xFE000000;
pub const PHYSICAL_DEVICE_SPACE: usize = V2P!(DEVICE_SPACE);
pub const KERNEL_BASE: usize = 0x80000000;
pub const KERNEL_LINK: usize = KERNEL_BASE + EXTENDED_MEMORY;

pub const PAGE_DIR_SHIFT: usize = 22;
pub const PAGE_TABLE_SHIFT: usize = 12;

pub const PTE_P: usize = 0x001;
pub const PTE_W: usize = 0x002;
pub const PTE_U: usize = 0x004;
pub const PTE_PS: usize = 0x080;

/// Heap Definitions
pub const HEAP_PAGES: usize = 25;
pub const STACK_PAGES: usize = 4;

pub struct LinkedListAllocator {
    pub head: StaticLinkedListNode,
}

#[derive(Debug, Clone)]
pub struct GlobalDescriptorTable {
    pub table: [u64; N_DESCRIPTORS], // Segment Descriptor List
    pub len: usize,                  // Size of GDT
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct GlobalDescriptorTablePointer {
    pub size: u16,
    pub base: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryLayoutEntry {
    pub virt: *const usize, // Start of the virtual address
    pub phys_start: usize,  // Start of the physical address
    pub phys_end: usize,    // End of the physical address
    pub perm: usize,        // Permission flags
}

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub address: *const usize,
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub start: usize,
    pub index: usize,
    pub end: usize,
}

bitflags! {
    pub struct DescriptorFlags: u64 {
        // Access
        const ACCESSED          = 1 << 40;
        const WRITABLE          = 1 << 41;
        const CONFORMING        = 1 << 42;
        const EXECUTABLE        = 1 << 43;
        const USER_SEGMENT      = 1 << 44;
        const DPL_RING_3        = 3 << 45;
        const PRESENT           = 1 << 47;
        const AVAILABLE         = 1 << 52;

        // Flags
        const LONG_MODE         = 1 << 53;
        const DEFAULT_SIZE      = 1 << 54;
        const GRANULARITY       = 1 << 55;

        // Limit
        const LIMIT_0_15        = 0xFFFF;
        const LIMIT_16_19       = 0xF << 48;

        // Base
        const BASE_0_23         = 0xFF_FFFF << 16;
        const BASE_24_31        = 0xFF << 56;
    }
}

// Common segments
pub const KERNEL_CODE_SEGMENT: u64 = DescriptorFlags::KERNEL_CODE32.bits();
pub const KERNEL_DATA_SEGMENT: u64 = DescriptorFlags::KERNEL_DATA.bits();
pub const USER_CODE_SEGMENT: u64 = DescriptorFlags::USER_CODE64.bits();
pub const USER_DATA_SEGMENT: u64 = DescriptorFlags::USER_DATA.bits();
