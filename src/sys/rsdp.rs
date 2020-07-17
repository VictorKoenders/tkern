use core::ops::Range;

#[repr(C)]
struct Descriptor {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt_address: u32,
}

pub fn init() {
    if let Some(_descriptor) = search(0xE_0000..0xF_FFFF) {}
}

fn search(range: Range<usize>) -> Option<&'static Descriptor> {
    for addr in range.step_by(16) {
        let rsdp = unsafe { &*(addr as *const Descriptor) };
        if &rsdp.signature == b"RSD PTR " {
            vga_println!("Found RSDP at {:?}", addr as *const ());
            return Some(rsdp);
        }
    }
    vga_println!("Did not find RSDP");
    None
}
