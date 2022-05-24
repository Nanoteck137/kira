//! Module to handle CPU instructions

use super::Reg;

#[derive(Debug)]
pub enum Instruction {
    Jal { rd: Reg, imm: i32 },

    Unknown(u32),
}

impl Instruction {
    pub fn decode(inst: u32) -> Instruction {
        let opcode = inst & 0b1111111;
        println!("Opcode: 0b{:b}", opcode);

        let test = JType::from(inst);
        println!("Test: {:?}", test);

        Instruction::Unknown(inst)
    }
}

#[derive(Copy, Clone, Debug)]
struct RType {
    funct7: u32,
    funct3: u32,
    rd: Reg,
    rs1: Reg,
    rs2: Reg,
}

impl From<u32> for RType {
    fn from(value: u32) -> Self {
        let funct7 = (value >> 25) & 0x7f;

        let rs2 = Reg::from((value >> 20) & 0x1f);
        let rs1 = Reg::from((value >> 15) & 0x1f);

        let funct3 = (value >> 12) & 0x7;

        let rd = Reg::from((value >> 7) & 0x1f);

        Self {
            funct7,
            funct3,

            rd,
            rs1,
            rs2
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct IType {
    imm: i32,
    funct3: u32,
    rd: Reg,
    rs1: Reg,
}

impl From<u32> for IType {
    fn from(value: u32) -> Self {
        let imm = (value as i32) >> 20;

        let rs1 = Reg::from((value >> 15) & 0x1f);
        let funct3 = (value >> 12) & 0x7;
        let rd = Reg::from((value >> 7) & 0x1f);

        Self {
            imm,
            funct3,
            rd,
            rs1
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct SType {
    imm: i32,
    funct3: u32,

    rs1: Reg,
    rs2: Reg,
}

impl From<u32> for SType {
    fn from(value: u32) -> Self {
        let imm115 = (value >> 25) & 0x7f;
        let imm40 = (value >> 7) & 0x1f;

        let imm = (imm115 << 5) | imm40;
        let imm = ((imm as i32) << 20) >> 20;

        let funct3 = (value >> 12) & 0x7;
        let rs1 = Reg::from((value >> 15) & 0x1f);
        let rs2 = Reg::from((value >> 20) & 0x1f);

        Self {
            imm,
            funct3,

            rs1,
            rs2
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct BType {
    imm: i32,
    funct3: u32,

    rs1: Reg,
    rs2: Reg,
}

impl From<u32> for BType {
    fn from(value: u32) -> Self {
        let imm12  = (value >> 31) & 0x1;
        let imm105 = (value >> 25) & 0x3f;
        let imm41  = (value >> 8)  & 0xf;
        let imm11  = (value >> 7)  & 0x1;

        let imm = (imm12 << 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1);
        let imm = ((imm as i32) << 19) >> 19;

        let funct3 = (value >> 12) & 0x7;

        let rs1 = Reg::from((value >> 15) & 0x1f);
        let rs2 = Reg::from((value >> 20) & 0x1f);

        Self {
            imm,
            funct3,

            rs1,
            rs2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct UType {
    imm: i32,
    rd: Reg,
}


impl From<u32> for UType {
    fn from(value: u32) -> Self {
        let imm = (value & !0xfff) as i32;
        let rd = Reg::from((value >> 7) & 0x1f);

        Self {
            imm,
            rd
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct JType {
    imm: i32,
    rd: Reg,
}

impl From<u32> for JType {
    fn from(value: u32) -> Self {
        let imm20   = (value >> 31) & 0x1;
        let imm101  = (value >> 21) & 0x3ff;
        let imm11   = (value >> 20) & 0x1;
        let imm1912 = (value >> 12) & 0xff;

        let imm = (imm20 << 20) | (imm1912 << 12) |
                  (imm11 << 11) | (imm101 << 1);
        let imm = ((imm as i32) << 11) >> 11;

        let rd = Reg::from((value >> 7) & 0x1f);

        Self {
            imm,
            rd
        }
    }
}
