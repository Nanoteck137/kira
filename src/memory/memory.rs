pub enum TypeWidth {
    // u8
    Byte,
    // u16
    HalfWord,
    // u32
    Word,
    // u64
    DoubleWord
}

pub trait Mmu {
    /// Read from memory
    fn read(&self, addr: u64, width: TypeWidth) -> u64;

    /// Write to memory
    fn write(&mut self, addr: u64, value: u64, width: TypeWidth);

    fn read_u8(&self, addr: u64) -> u8 {
        self.read(addr, TypeWidth::Byte) as u8
    }

    fn read_u16(&self, addr: u64) -> u16 {
        self.read(addr, TypeWidth::HalfWord) as u16
    }

    fn read_u32(&self, addr: u64) -> u32 {
        self.read(addr, TypeWidth::Word) as u32
    }

    fn read_u64(&self, addr: u64) -> u64 {
        self.read(addr, TypeWidth::DoubleWord) as u64
    }

    fn write_u8(&mut self, addr: u64, value: u8) {
        self.write(addr, value as u64, TypeWidth::Byte);
    }

    fn write_u16(&mut self, addr: u64, value: u16) {
        self.write(addr, value as u64, TypeWidth::HalfWord);
    }

    fn write_u32(&mut self, addr: u64, value: u32) {
        self.write(addr, value as u64, TypeWidth::Word);
    }

    fn write_u64(&mut self, addr: u64, value: u64) {
        self.write(addr, value as u64, TypeWidth::DoubleWord);
    }
}