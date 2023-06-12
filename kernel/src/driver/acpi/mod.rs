use crate::println;
use alloc::string::String;

pub mod madt;

#[repr(C, packed)]
pub struct RSDP {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
}

#[repr(C, packed)]
pub struct ExtendedRSDP {
    first_part: RSDP,
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    _reserved: [u8; 3],
}

#[repr(C, packed)]
pub struct AcpiSdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oemid: [u8; 6],
    oemtable_id: [u8; 8],
    oemrevision: u32,
    creator_id: u32,
    creator_revision: u32,
}

#[repr(C, packed)]
pub struct RsdtXsdt {
    header: AcpiSdtHeader,
    pointer_to_other_sdt: [u64; 0],
}

pub struct AcpiSdt<SDT> {
    pub header: AcpiSdtHeader,
    pub sdt: SDT,
}

pub fn rsdp(address: usize) -> *const RSDP {
    address as *const RSDP
}

fn version_2(rsdp: &ExtendedRSDP) -> bool {
    rsdp.first_part.revision >= 2
}

pub fn rstd_xstd(rsdp: &ExtendedRSDP, physical_memory_offset: usize) -> *const RsdtXsdt {
    if version_2(&rsdp) {
        (rsdp.xsdt_address as usize + physical_memory_offset) as *const RsdtXsdt
    } else {
        (rsdp.first_part.rsdt_address as usize + physical_memory_offset) as *const RsdtXsdt
    }
}

pub fn find_table<SDT>(
    rsdt_xsdt: &RsdtXsdt,
    signature: &[u8],
    physical_memory_offset: usize,
) -> Option<*const AcpiSdt<SDT>> {
    let size = core::mem::size_of::<AcpiSdtHeader>();
    let num_entries = (rsdt_xsdt.header.length as usize - size) / core::mem::size_of::<usize>();
    let sdts = unsafe { (rsdt_xsdt as *const _ as *const u8).add(size) } as *const u64;

    for i in 0..num_entries {
        let pointer = unsafe { sdts.add(i).read_unaligned() as usize } + physical_memory_offset;
        let header = pointer as *const AcpiSdtHeader;

        if header.is_null() {
            continue;
        }

        let entry = unsafe { &*header };
        println!("{}", String::from_utf8_lossy(&entry.signature));

        if entry.signature == *signature {
            return Some(header as *const AcpiSdt<SDT>);
        }
    }
    None
}
