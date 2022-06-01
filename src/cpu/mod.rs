//! CPU Module

use crate::memory::Mmu;

use instruction::Instruction;
pub use cpu::{ Hart, Reg };

mod instruction;
mod cpu;

const MAX_CONTROL_REGISTERS: usize = 4096;

pub struct SimpleHart {
    registers: [u64; 33],
    csr: [u64; MAX_CONTROL_REGISTERS],
    pub mmu: Box<dyn Mmu>,
}

impl SimpleHart {
    pub fn new(mmu: Box<dyn Mmu>) -> Self {
        Self {
            registers: [0u64; 33],
            csr: [0u64; MAX_CONTROL_REGISTERS],
            mmu,
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
        //println!("Executing CPU Instruction: {:x?}", inst);

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
                let target = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let return_addr = self.reg(Reg::Pc);
                self.set_reg(rd, return_addr);

                self.set_reg(Reg::Pc, target);
            }

            Instruction::Beq  { rs1, rs2, imm } => { 
                if self.reg(rs1) == self.reg(rs2) {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }

            Instruction::Bne  { rs1, rs2, imm } => { 
                if self.reg(rs1) != self.reg(rs2) {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }

            Instruction::Blt  { rs1, rs2, imm } => {
                if (self.reg(rs1) as i64) < (self.reg(rs2) as i64) {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }
            Instruction::Bge  { rs1, rs2, imm } => { 
                if self.reg(rs1) as i64 >= self.reg(rs2) as i64 {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }

            Instruction::Bltu { rs1, rs2, imm } => {
                if self.reg(rs1) < self.reg(rs2) {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }

            Instruction::Bgeu { rs1, rs2, imm } => {
                if self.reg(rs1) >= self.reg(rs2) {
                    let target = current_pc.wrapping_add(imm as i64 as u64);
                    self.set_reg(Reg::Pc, target);
                }
            }

            Instruction::Lb  { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let result = self.mmu.read_u8(addr);
                self.set_reg(rd, result as i8 as i64 as u64);
            }

            Instruction::Lh  { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let result = self.mmu.read_u16(addr);
                self.set_reg(rd, result as i16 as i64 as u64);
            }

            Instruction::Lw  { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let result = self.mmu.read_u32(addr);
                self.set_reg(rd, result as i32 as i64 as u64);
            }

            Instruction::Lbu { rd, rs1, imm } => { 
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let result = self.mmu.read_u8(addr);
                self.set_reg(rd, result as u64);
            }

            Instruction::Lhu { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let result = self.mmu.read_u16(addr);
                self.set_reg(rd, result as u64);
            }

            Instruction::Lwu { rd, rs1, imm } => { 
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let result = self.mmu.read_u32(addr);
                self.set_reg(rd, result as u64);
            }

            Instruction::Ld  { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let result = self.mmu.read_u64(addr);
                self.set_reg(rd, result as i64 as u64);
            }

            Instruction::Sb { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let value = self.reg(rs2) as u8;
                self.mmu.write_u8(addr, value);
            }

            Instruction::Sh { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let value = self.reg(rs2) as u16;
                self.mmu.write_u16(addr, value);
            }

            Instruction::Sw { rs1, rs2, imm } => { 
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let value = self.reg(rs2) as u32;
                self.mmu.write_u32(addr, value);
            }

            Instruction::Sd { rs1, rs2, imm } => { 
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                let value = self.reg(rs2);
                self.mmu.write_u64(addr, value);
            }

            Instruction::Addi  { rd, rs1, imm } => { 
                let res = self.reg(rs1).wrapping_add(imm as i64 as u64);
                self.set_reg(rd, res);
            }

            Instruction::Slti  { rd, rs1, imm } => {
                if (self.reg(rs1) as i64) < (imm as i64) {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }
            }

            Instruction::Sltiu { rd, rs1, imm } => { 
                if self.reg(rs1) < (imm as i64 as u64) {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }
            }

            Instruction::Xori  { rd, rs1, imm } => {
                let result = self.reg(rs1) ^ (imm as i64 as u64);
                self.set_reg(rd, result);
            }

            Instruction::Ori   { rd, rs1, imm } => { 
                let res = self.reg(rs1) | imm as i64 as u64;
                self.set_reg(rd, res);
            }

            Instruction::Andi  { rd, rs1, imm } => {
                let result = self.reg(rs1) & imm as i64 as u64;
                self.set_reg(rd, result);
            }

            Instruction::Slli  { rd, rs1, shamt } => { 
                // TODO(patrik): Wrapping?
                let result = self.reg(rs1) << shamt;
                self.set_reg(rd, result);
            }

            Instruction::Srli  { rd, rs1, shamt } => {
                let result = self.reg(rs1) >> shamt;
                self.set_reg(rd, result);
            }

            Instruction::Srai  { rd, rs1, shamt } => {
                let result = (self.reg(rs1) as i64) >> shamt;
                self.set_reg(rd, result as u64);
            }

            Instruction::Addiw { rd, rs1, imm } => { 
                let result = (self.reg(rs1) as u32)
                    .wrapping_add(imm as u32);
                self.set_reg(rd, result as i32 as i64 as u64);
            }

            Instruction::Slliw { rd, rs1, shamt } => {
                let result = (self.reg(rs1) as u32) << shamt;
                self.set_reg(rd, result as i32 as i64 as u64);
            }

            Instruction::Srliw { rd, rs1, shamt } => {
                let result = (self.reg(rs1) as u32) >> shamt;
                self.set_reg(rd, result as i32 as u64);                
            }

            Instruction::Sraiw { rd, rs1, shamt } => {
                let result = (self.reg(rs1) as u32 as i32) >> shamt;
                self.set_reg(rd, result as i64 as u64);                
            }

            Instruction::Add  { rd, rs1, rs2 } => { 
                let result = self.reg(rs1).wrapping_add(self.reg(rs2));
                self.set_reg(rd, result);
            }

            Instruction::Sub  { rd, rs1, rs2 } => {
                let result = self.reg(rs1).wrapping_sub(self.reg(rs2));
                self.set_reg(rd, result);
            }

            Instruction::Sll  { rd, rs1, rs2 } => {
                let shamt = self.reg(rs2) & 0x3f;
                let result = self.reg(rs1) << shamt;
                self.set_reg(rd, result);
            }

            Instruction::Slt  { rd, rs1, rs2 } => {
                if (self.reg(rs1) as i64) < (self.reg(rs2) as i64) {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0)
                }
            }

            Instruction::Sltu { rd, rs1, rs2 } => {
                if self.reg(rs1) < self.reg(rs2) {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0)
                }
            }

            Instruction::Xor  { rd, rs1, rs2 } => {
                let result = self.reg(rs1) ^ self.reg(rs2);
                self.set_reg(rd, result);
            }

            Instruction::Srl  { rd, rs1, rs2 } => {
                let shamt = self.reg(rs2) & 0x3f;
                let result = self.reg(rs1) >> shamt;
                self.set_reg(rd, result);
            }

            Instruction::Sra  { rd, rs1, rs2 } => {
                let shamt = self.reg(rs2) & 0x3f;
                let result = (self.reg(rs1) as i64) >> shamt;
                self.set_reg(rd, result as u64);
            }

            Instruction::Or   { rd, rs1, rs2 } => {
                let result = self.reg(rs1) | self.reg(rs2);
                self.set_reg(rd, result);
            }

            Instruction::And  { rd, rs1, rs2 } => {
                let result = self.reg(rs1) & self.reg(rs2);
                self.set_reg(rd, result);
            }

            Instruction::Addw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;
                let result = rs1.wrapping_add(rs2);
                self.set_reg(rd, result as i32 as i64 as u64);
            }

            Instruction::Subw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;
                let result = rs1.wrapping_sub(rs2);
                self.set_reg(rd, result as i32 as i64 as u64);
            }

            Instruction::Sllw { rd, rs1, rs2 } => {
                let shamt = (self.reg(rs2) & 0x1f) as u32;
                let result = (self.reg(rs1) as u32) << shamt;
                self.set_reg(rd, result as i32 as i64 as u64);
            }

            Instruction::Srlw { rd, rs1, rs2 } => {
                let shamt = self.reg(rs2) & 0x1f;
                let result = (self.reg(rs1) as u32) >> shamt;
                self.set_reg(rd, result as i32 as i64 as u64);
            }

            Instruction::Sraw { rd, rs1, rs2 } => {
                let shamt = self.reg(rs2) & 0x1f;
                let result = (self.reg(rs1) as u32 as i32) >> shamt;
                self.set_reg(rd, result as i64 as u64);
            }

            Instruction::Fence {} => { }

            Instruction::Ecall => {
                const CSR_MTVEC: u16 = 0x305;

                let pc = self.csr[CSR_MTVEC as usize];
                self.set_reg(Reg::Pc, pc);
            }

            Instruction::Ebreak => { todo!(); }
            Instruction::Sret => { todo!(); }
            Instruction::Mret => {
                // TODO(patrik): Not correct 
                const CSR_MEPC: u16 = 0x341;
                const CSR_MCAUSE: u16 = 0x342;

                self.csr[CSR_MCAUSE as usize] = 11;

                let pc = self.csr[CSR_MEPC as usize];
                self.set_reg(Reg::Pc, pc);
            }

            Instruction::Csrrw { rd, rs1, csr } => {
                if rd != Reg::X0 {
                    let old = self.csr[csr as usize];
                    self.set_reg(rd, old);
                }

                self.csr[csr as usize] = self.reg(rs1);
            }

            Instruction::Csrrs { rd, rs1, csr } => {
                let old = self.csr[csr as usize];
                self.set_reg(rd, old);

                if rs1 != Reg::X0 {
                    self.csr[csr as usize] = old | self.reg(rs1);
                }
            }

            Instruction::Csrrc { rd, rs1, csr } => { todo!(); }

            Instruction::Csrrwi { rd, uimm, csr } => {
                if rd != Reg::X0 {
                    let old = self.csr[csr as usize];
                    self.set_reg(rd, old);
                }

                self.csr[csr as usize] = uimm as u64;
            }

            Instruction::Csrrsi { rd, uimm, csr } => { todo!(); }
            Instruction::Csrrci { rd, uimm, csr } => { todo!(); }

            /*
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
            */
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

impl Hart for SimpleHart {
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
        // println!("{:#x}: {:#x}", pc, inst);

        match Instruction::decode(inst) {
            Ok(inst) => self.execute_instruction(pc, inst),
            Err(e) => panic!("Failed to decode inst: {:#x} {:x?}", pc, e),
        }
    }
}
