#![allow(dead_code)]

use std::path::Path;
use std::fs::File;
use std::io::Read;

use memory::{ Memory, Mmu };

mod elf;
mod memory;

fn read_file_to_vec<P>(path: P) -> Vec<u8>
    where P: AsRef<Path>
{
    let mut file = File::open(path).unwrap();

    let mut result = Vec::new();
    file.read_to_end(&mut result).unwrap();

    result
}

enum Reg {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,

    Pc
}

impl Reg {
    fn index(&self) -> usize {
        return match self {
            Reg::X0 => 0,
            Reg::X1 => 1,
            Reg::X2 => 2,
            Reg::X3 => 3,
            Reg::X4 => 4,
            Reg::X5 => 5,
            Reg::X6 => 6,
            Reg::X7 => 7,
            Reg::X8 => 8,
            Reg::X9 => 9,
            Reg::X10 => 10,
            Reg::X11 => 11,
            Reg::X12 => 12,
            Reg::X13 => 13,
            Reg::X14 => 14,
            Reg::X15 => 15,
            Reg::X16 => 16,
            Reg::X17 => 17,
            Reg::X18 => 18,
            Reg::X19 => 19,
            Reg::X20 => 20,
            Reg::X21 => 21,
            Reg::X22 => 22,
            Reg::X23 => 23,
            Reg::X24 => 24,
            Reg::X25 => 25,
            Reg::X26 => 26,
            Reg::X27 => 27,
            Reg::X28 => 28,
            Reg::X29 => 29,
            Reg::X30 => 30,
            Reg::X31 => 31,

            Reg::Pc => 32,
        };
    }
}

struct Hart {
    registers: [u64; 33],
    mmu: Mmu
}

impl Hart {
    fn new(mmu: Mmu) -> Self {
        Self {
            registers: [0u64; 33],
            mmu,
        }
    }

    /// Get register inside the cpu with name
    fn reg(&self, reg: Reg) -> u64 {
        self.regi(reg.index())
    }

    /// Set register inside the cpu with name
    fn set_reg(&mut self, reg: Reg, val: u64) {
        self.set_regi(reg.index(), val);
    }

    /// Get register inside the cpu with index
    fn regi(&self, reg: usize) -> u64 {
        self.registers[reg]
    }

    /// Set register inside the cpu with index
    fn set_regi(&mut self, reg: usize, val: u64) {
        self.registers[reg] = val;
    }

    fn fetch_u32(&mut self) -> u32 {
        let pc = self.reg(Reg::Pc);
        // TODO(patrik): Read from memory
        let res = self.mmu.read_u32(pc);
        self.set_reg(Reg::Pc, pc + 4);

        res
    }

    fn step(&mut self) {
        let pc = self.reg(Reg::Pc);
        let inst = self.fetch_u32();
        println!("{:#x}: {:#x}", pc, inst);

        let opcode = inst & 0b1111111;
        println!("Opcode: 0b{:b}", opcode);

        match opcode {
            0b1101111 => {
                // JAL
                let rd = (inst >> 7) & 0b11111;
                println!("RD: {}", rd);

                let raw_imm = (inst & !0xfff) >> 12;
                println!("Raw imm: {:#x}", raw_imm);

                let imm1912 = (raw_imm >> 0) & 0b11111111;
                let imm11 = (raw_imm >> 8) & 0b1;
                let imm101 = (raw_imm >> 9) & 0b1111111111;
                let imm20 = (raw_imm >> 19) & 0b1;
                let imm = (imm20 << 20) | (imm1912 << 12) | (imm11 << 11) | (imm101 << 1);
                let imm = ((imm as i32) << 11) >> 11;
                println!("Imm: {:#x}", imm);
                println!("New PC: {:#x}", pc + imm as u64);

                self.set_reg(Reg::Pc, pc + imm as u64);
            }

            _ => {
                panic!("Unknown opcode: 0b{:b}", opcode);
            }
        }
    }
}


fn main() {
    let path = "/opt/riscv/target/share/riscv-tests/isa/rv64ui-v-add";
    let file_data = read_file_to_vec(path);
    println!("Data: {}", file_data.len());

    let e = elf::Elf::parse(&file_data).unwrap();
    println!("Elf: {:#?}", e);

    let memory = Memory::new(100 * 1024 * 1024);
    let mut mmu = Mmu::new(memory);

    for program_header in e.program_header_iter() {
        if program_header.typ() == elf::ProgramHeaderTyp::Load {
            let data = e.program_header_data(program_header)
                .expect("Failed to get program header data");
            println!("{:#x?}: {:#x}", program_header, data.len());

            for index in 0..data.len() {
                let addr = program_header.vaddr() + index as u64;
                let value = data[index];
                mmu.write_u8(addr, value);
            }
        }
    }

    let mut hart = Hart::new(mmu);

    hart.set_reg(Reg::X0, 0x1337);
    hart.set_reg(Reg::Pc, e.entry());

    println!("X0: {:#x}", hart.reg(Reg::X0));
    hart.step();
}
