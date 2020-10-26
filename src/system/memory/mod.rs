mod address;
mod paging;
pub mod mapping;

pub use self::address::{AddressAccess, PhysicalAddress, VirtualAddress};

pub struct Memory {

}

impl Memory {
    pub fn init() -> Memory {
        unsafe { self::mapping::init(); }
        Memory {
        }
    }
}