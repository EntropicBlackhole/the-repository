use core::cell::OnceCell;

use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use spin::Mutex;
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, PageTable, PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

#[derive(Debug)]
pub struct Memory {
    pub mapper: OffsetPageTable<'static>,
    pub frame_allocator: BootInfoFrameAllocator,
}

unsafe impl Sync for Memory {}
unsafe impl Send for Memory {}

pub static MEMORY: Mutex<OnceCell<Memory>> = Mutex::new(OnceCell::new());

impl Memory {
    pub fn identity_map(&mut self, physical_address: usize, flags: Option<PageTableFlags>) {
        let flags = flags.unwrap_or_else(|| {
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE
        });
        let physical_address = PhysAddr::new(physical_address as u64);
        let physical_frame: PhysFrame = PhysFrame::containing_address(physical_address);
        unsafe {
            self.mapper
                .identity_map(physical_frame, flags, &mut self.frame_allocator)
                .expect("Failed to identity map")
                .flush();
        }
    }
}

pub fn identity_map(physical_address: usize, flags: Option<PageTableFlags>) {
    let mut lock = MEMORY.lock();
    (*lock)
        .get_mut()
        .expect("`MEMORY` accessed before call to init")
        .identity_map(physical_address, flags);
}

/// Initialize a new OffsetPageTable and create a FrameAllocator using the provided memory map and physical memory offset.
///
/// This function is unsafe because the caller must ensure the following:
/// - The complete physical memory is mapped to virtual memory at the passed `physical_memory_offset`.
/// - The passed memory map is valid, and all frames marked as `USABLE` in the memory map are really unused.
/// - This function is called only once to avoid aliasing `&mut` references, which would result in undefined behavior.
///
/// # Safety
///
/// The caller must ensure the following:
/// - The complete physical memory is mapped to virtual memory at `physical_memory_offset`.
/// - The passed memory map is valid.
/// - All frames marked as `USABLE` in the memory map are really unused.
/// - This function is called only once to avoid aliasing `&mut` references.
///
/// # Parameters
/// - `physical_memory_offset`: The offset specifying the mapping of physical memory to virtual memory.
/// - `memory_map`: The memory map providing information about the available frames.
pub unsafe fn init(physical_memory_offset: VirtAddr, memory_map: &'static MemoryRegions) {
    let level_4_table = active_level_4_table(physical_memory_offset);
    let mapper = OffsetPageTable::new(level_4_table, physical_memory_offset);
    let frame_allocator = BootInfoFrameAllocator::init(memory_map);

    MEMORY
        .lock()
        .set(Memory {
            mapper,
            frame_allocator,
        })
        .unwrap();
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
#[derive(Debug)]
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.kind == MemoryRegionKind::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.start..r.end);
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
