//! CPU Module

use crate::memory::Mmu;

use instruction::Instruction;
pub use cpu::{ Hart, Reg };

mod instruction;
mod cpu;

pub struct TestingHart {
    registers: [u64; 33],
    mmu: Box<dyn Mmu>,

    mepc: u64,
}

impl TestingHart {
    pub fn new(mmu: Box<dyn Mmu>) -> Self {
        Self {
            registers: [0u64; 33],
            mmu,
            mepc: 0,
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
            Instruction::Lui { rd, imm } => {
                self.set_reg(rd, imm as i64 as u64);
            }

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


            Instruction::Sd { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let value = self.reg(rs2);
                println!("Writing: {:#x} -> {:#x}", value, addr);
                self.mmu.write_u64(addr, value);
            }

            Instruction::Bne { rs1, rs2, imm } => {
                if self.reg(rs1) != self.reg(rs2) {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }

            Instruction::Beq { rs1, rs2, imm } => {
                if self.reg(rs1) == self.reg(rs2) {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }

            Instruction::Bge { rs1, rs2, imm } => {
                if self.reg(rs1) >= self.reg(rs2) {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }

            Instruction::Addi { rd, rs1, imm } => {
                let res = self.reg(rs1).wrapping_add(imm as i64 as u64);
                self.set_reg(rd, res);
            }

            Instruction::Ori { rd, rs1, imm } => {
                let res = self.reg(rs1) | imm as i64 as u64;
                self.set_reg(rd, res);
            }

            Instruction::Addiw { rd, rs1, imm } => {
                let result = (self.reg(rs1) as u32)
                    .wrapping_add(imm as u32);
                self.set_reg(rd, result as i32 as i64 as u64);
            }

            Instruction::Slli { rd, rs1, shamt } => {
                // TODO(patrik): Wrapping?
                let result = self.reg(rs1) << shamt;
                self.set_reg(rd, result);
            }

            Instruction::Srli { rd, rs1, shamt } => {
                let result = self.reg(rs1) >> shamt;
                self.set_reg(rd, result);
            }

            Instruction::Add { rd, rs1, rs2 } => {
                let result = self.reg(rs1).wrapping_add(self.reg(rs2));
                self.set_reg(rd, result);
            }

            Instruction::Mret => {
                self.set_reg(Reg::Pc, self.mepc);
            }

            Instruction::Csrrw { rd, rs1, csr } => {
                self.set_reg(rd, 0);
                if csr == 0x341 {
                    self.mepc = self.reg(rs1);
                    println!("Setting MEPC: {:#x?}", self.mepc);
                }
                println!("TODO: csrrw: {:?} {:?} {:#x}", rd, rs1, csr);
            }

            Instruction::Csrrs { rd, rs1, csr } => {
                self.set_reg(rd, 0);
                println!("TODO: csrrs: {:?} {:?} {:#x}", rd, rs1, csr);
            }

            Instruction::Csrrwi { rd, uimm, csr } => {
                self.set_reg(rd, 0);
                println!("TODO: csrrs: {:?} {:?} {:#x}", rd, uimm, csr);
            }

            Instruction::Fence {} => {
                println!("Fence");
            }

            Instruction::Ecall => {
                self.dump();
                if self.reg(Reg::X10) != 0 {
                    panic!("Test Failed: #{}", self.reg(Reg::X10));
                }

                panic!("Syscall");
            }

            _ => panic!("Not implemented: {:x?}", inst),
        }
    }

    pub fn dump(&self) {
        for i in 0..32 {
            if i % 4 == 0 && i != 0 { println!(); }
            print!("x{:02}: {:016x} ", i, self.registers[i]);
        }
        println!();
        println!("Pc: {:016x}", self.reg(Reg::Pc));
    }
}

impl Hart for TestingHart {
    /// Get value from register
    fn reg(&self, reg: Reg) -> u64 {
        self.registers[reg.index()]
    }

    /// Set register to value
    fn set_reg(&mut self, reg: Reg, val: u64) {
        if reg != Reg::X0 {
            self.registers[reg.index()] = val;
        }
    }

    /// Step the hart one instruction
    fn step(&mut self) {
        let pc = self.reg(Reg::Pc);
        let inst = self.fetch_u32();
        println!("{:#x}: {:#x}", pc, inst);

        match Instruction::decode(inst) {
            Ok(inst) => self.execute_instruction(pc, inst),
            Err(e) => panic!("Failed to decode inst: {:?}", e),
        }
    }
}
