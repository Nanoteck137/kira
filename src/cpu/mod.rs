//! CPU Module

use crate::memory::Mmu;

use instruction::Instruction;

mod instruction;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Reg {
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

impl From<u32> for Reg {
    fn from(value: u32) -> Self {
        match value {
            0 => Reg::X0,
            1 => Reg::X1,
            2 => Reg::X2,
            3 => Reg::X3,
            4 => Reg::X4,
            5 => Reg::X5,
            6 => Reg::X6,
            7 => Reg::X7,
            8 => Reg::X8,
            9 => Reg::X9,
            10 => Reg::X10,
            11 => Reg::X11,
            12 => Reg::X12,
            13 => Reg::X13,
            14 => Reg::X14,
            15 => Reg::X15,
            16 => Reg::X16,
            17 => Reg::X17,
            18 => Reg::X18,
            19 => Reg::X19,
            20 => Reg::X20,
            21 => Reg::X21,
            22 => Reg::X22,
            23 => Reg::X23,
            24 => Reg::X24,
            25 => Reg::X25,
            26 => Reg::X26,
            27 => Reg::X27,
            28 => Reg::X28,
            29 => Reg::X29,
            30 => Reg::X30,
            31 => Reg::X31,

            32 => Reg::Pc,

            _ => panic!("Unknown value: {}", value),
        }
    }
}

pub struct Hart {
    registers: [u64; 33],
    mmu: Mmu
}

impl Hart {
    pub fn new(mmu: Mmu) -> Self {
        Self {
            registers: [0u64; 33],
            mmu,
        }
    }

    /// Get register inside the cpu
    pub fn reg(&self, reg: Reg) -> u64 {
        self.registers[reg.index()]
    }

    /// Set register inside the cpu
    pub fn set_reg(&mut self, reg: Reg, val: u64) {
        if reg != Reg::X0 {
            self.registers[reg.index()] = val;
        }
    }

    fn fetch_u32(&mut self) -> u32 {
        let pc = self.reg(Reg::Pc);
        // TODO(patrik): Read from memory
        let res = self.mmu.read_u32(pc);
        self.set_reg(Reg::Pc, pc + 4);

        res
    }

    fn execute_instruction(&mut self, current_pc: u64, inst: Instruction) {
        println!("Executing CPU Instruction: {:x?}", inst);

        match inst {
            /*
            Instruction::Auipc { rd, imm } => {
                let target = current_pc.wrapping_add(imm as i64 as u64);
                self.set_reg(rd, target);
            }

            Instruction::Jal { rd, imm } => {
                let target = current_pc.wrapping_add(imm as i64 as u64);
                let return_address = self.reg(Reg::Pc);

                self.set_reg(rd, return_address);
                self.set_reg(Reg::Pc, target);
            }

            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.reg(rs1).wrapping_add(imm as i64 as u64);

                let return_addr = self.reg(Reg::Pc);
                self.set_reg(rd, return_addr);
                self.set_reg(Reg::Pc, target);
            }

            Instruction::Addi { rd, rs1, imm } => {
                let res = self.reg(rs1).wrapping_add(imm as i64 as u64);
                self.set_reg(rd, res);
            }

            Instruction::Slli { rd, rs1, shamt } => {
                // TODO(patrik): Wrapping?
                let result = self.reg(rs1) << shamt;
                self.set_reg(rd, result);
            }

            Instruction::Add { rd, rs1, rs2 } => {
                let result = self.reg(rs1).wrapping_add(self.reg(rs2));
                self.set_reg(rd, result);
            }

            Instruction::Csrrw { rd, rs1, csr } => {
                println!("TODO: csrrw: {:?} {:?} {:#x}", rd, rs1, csr);
            }

            Instruction::Csrrs { rd, rs1, csr } => {
                println!("TODO: csrrs: {:?} {:?} {:#x}", rd, rs1, csr);
            }
            */

            _ => panic!("Not implemented: {:?}", inst),
        }
    }

    pub fn step(&mut self) {
        let pc = self.reg(Reg::Pc);
        let inst = self.fetch_u32();
        println!("{:#x}: {:#x}", pc, inst);

        match Instruction::decode(inst) {
            Ok(inst) => self.execute_instruction(pc, inst),
            Err(e) => panic!("Failed to decode inst: {:?}", e),
        }
    }

    pub fn dump(&self) {
        for i in 0..32 {
            if i % 4 == 0 && i != 0 { println!(); }
            print!("x{:02}: {:016x} ", i, self.registers[i]);
        }
        println!("Pc: {:016x}", self.reg(Reg::Pc));
    }
}
