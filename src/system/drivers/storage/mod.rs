pub enum Error {

}

pub trait Storage {
    fn device_count(&self) -> Result<u8, Error>;
    fn size(&self, index: u8) -> Result<usize, Error>;
    fn read(&self, index: u8, offset: usize, buffer: &mut [u8]) -> Result<usize, Error>;
    fn write(&self, index: u8, offset: usize, buffer: &[u8]) -> Result<usize, Error>;
}
