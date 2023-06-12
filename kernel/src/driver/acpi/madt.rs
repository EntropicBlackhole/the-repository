/// Only use with `ACPISDT<MADT>`
#[repr(C, packed)]
pub struct MADT {
    pub lapic_address: usize,
    pub flags: u32,
    pub madt_entries: [MADTEntryHeader; 0],
}

#[repr(C, packed)]
pub struct MADTEntryHeader {
    pub entry_type: u32,
    pub length: u32,
}
