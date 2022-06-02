//! Module to handle memory

pub use memory::{ Mmu, TypeWidth };

mod memory;

pub struct TestingMemory {
    memory: Vec<u8>,
}

impl TestingMemory {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size],
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.memory[addr] = value;
    }

    pub fn write_u16(&mut self, addr: usize, value: u16) {
        self.memory[addr + 0] = ((value >> 0)  & 0xff) as u8;
        self.memory[addr + 1] = ((value >> 8)  & 0xff) as u8;
    }

    pub fn write_u32(&mut self, addr: usize, value: u32) {
        self.memory[addr + 0] = ((value >> 0)  & 0xff) as u8;
        self.memory[addr + 1] = ((value >> 8)  & 0xff) as u8;
        self.memory[addr + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[addr + 3] = ((value >> 24) & 0xff) as u8;
    }

    pub fn write_u64(&mut self, addr: usize, value: u64) {
        self.memory[addr + 0] = ((value >> 0)  & 0xff) as u8;
        self.memory[addr + 1] = ((value >> 8)  & 0xff) as u8;
        self.memory[addr + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[addr + 3] = ((value >> 24) & 0xff) as u8;
        self.memory[addr + 4] = ((value >> 32) & 0xff) as u8;
        self.memory[addr + 5] = ((value >> 40) & 0xff) as u8;
        self.memory[addr + 6] = ((value >> 48) & 0xff) as u8;
        self.memory[addr + 7] = ((value >> 56) & 0xff) as u8;
    }

    pub fn read_u8(&self, addr: usize) -> u8 {
        self.memory[addr]
    }

    pub fn read_u16(&self, addr: usize) -> u16 {
        let v0 = self.memory[addr + 0] as u16;
        let v1 = self.memory[addr + 1] as u16;

        (v1 << 8) | v0
    }

    pub fn read_u32(&self, addr: usize) -> u32 {
        let v0 = self.memory[addr + 0] as u32;
        let v1 = self.memory[addr + 1] as u32;
        let v2 = self.memory[addr + 2] as u32;
        let v3 = self.memory[addr + 3] as u32;

        (v3 << 24) | (v2 << 16) | (v1 << 8) | v0
    }

    pub fn read_u64(&self, addr: usize) -> u64 {
        let v0 = self.memory[addr + 0] as u64;
        let v1 = self.memory[addr + 1] as u64;
        let v2 = self.memory[addr + 2] as u64;
        let v3 = self.memory[addr + 3] as u64;
        let v4 = self.memory[addr + 4] as u64;
        let v5 = self.memory[addr + 5] as u64;
        let v6 = self.memory[addr + 6] as u64;
        let v7 = self.memory[addr + 7] as u64;

        (v7 << 56) | (v6 << 48) | (v5 << 40) | (v4 << 32) |
        (v3 << 24) | (v2 << 16) | (v1 << 8)  | v0
    }
}

/// This is temporary, used for the tests
pub const MEMORY_OFFSET: u64 = 0x80000000;

pub struct TestingMmu {
    memory: TestingMemory,
}

impl TestingMmu {
    pub fn new(memory: TestingMemory) -> Self {
        Self {
            memory
        }
    }
}

impl Mmu for TestingMmu {
    /// Read from memory
    fn read(&self, addr: u64, width: TypeWidth) -> u64 {
        if addr >= MEMORY_OFFSET &&
            addr < MEMORY_OFFSET + self.memory.len() as u64
        {
            let addr = addr.wrapping_sub(MEMORY_OFFSET);
            let addr: usize = addr.try_into().unwrap();

            return match width {
                TypeWidth::Byte =>  
                    self.memory.read_u8(addr)  as u64,
                TypeWidth::HalfWord => 
                    self.memory.read_u16(addr) as u64,
                TypeWidth::Word => 
                    self.memory.read_u32(addr) as u64,
                TypeWidth::DoubleWord => 
                    self.memory.read_u64(addr) as u64,
            };
        }

        panic!("Unknown addr: {:#x}", addr);
    }

    /// Write to memory
    fn write(&mut self, addr: u64, value: u64, width: TypeWidth) {
        if addr == 0x1000 {
            print!("{}", value as u8 as char);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
            
            return;
        }

        if addr >= MEMORY_OFFSET &&
            addr < MEMORY_OFFSET + self.memory.len() as u64
        {
            let addr = addr.wrapping_sub(MEMORY_OFFSET);
            let addr: usize = addr.try_into().unwrap();

            return match width {
                TypeWidth::Byte => 
                    self.memory.write_u8(addr, value as u8),
                TypeWidth::HalfWord => 
                    self.memory.write_u16(addr, value as u16),
                TypeWidth::Word =>
                    self.memory.write_u32(addr, value as u32),
                TypeWidth::DoubleWord => 
                    self.memory.write_u64(addr, value as u64),
            };
        }

        panic!("Unknown addr: {:#x}", addr);
    }
}