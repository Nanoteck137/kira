//! Module to handle CPU instructions

use super::Reg;

#[derive(Debug)]
pub enum Error {
    UnknownOpcode(u32),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Opcode {
    Lui,
    Auipc,
    Jal,
    Jalr,
    Branch,
    Load,
    Store,
    OpImm,
    OpImm32,
    Op,
    Op32,
    MiscMem,
    System,
}

impl TryFrom<u32> for Opcode {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self> {
        return match value {
            0b0110111 => Ok(Self::Lui),
            0b0010111 => Ok(Self::Auipc),
            0b1101111 => Ok(Self::Jal),
            0b1100111 => Ok(Self::Jalr),
            0b1100011 => Ok(Self::Branch),
            0b0000011 => Ok(Self::Load),
            0b0100011 => Ok(Self::Store),
            0b0010011 => Ok(Self::OpImm),
            0b0011011 => Ok(Self::OpImm32),
            0b0110011 => Ok(Self::Op),
            0b0111011 => Ok(Self::Op32),
            0b0001111 => Ok(Self::MiscMem),
            0b1110011 => Ok(Self::System),

            _ => Err(Error::UnknownOpcode(value)),
        };
    }
}

#[derive(Debug)]
pub enum Instruction {
    /// Opcode: LUI
    Lui { rd: Reg, imm: i32 },

    /// Opcode: AUIPC
    Auipc { rd: Reg, imm: i32 },

    /// Opcode: JAL
    Jal { rd: Reg, imm: i32 },

    /// Opcode:: JALR
    Jalr { rd: Reg, rs1: Reg, imm: i32 },

    /// Opcode: BRANCH
    Beq  { rs1: Reg, rs2: Reg, imm: i32 },
    Bne  { rs1: Reg, rs2: Reg, imm: i32 },
    Blt  { rs1: Reg, rs2: Reg, imm: i32 },
    Bge  { rs1: Reg, rs2: Reg, imm: i32 },
    Bltu { rs1: Reg, rs2: Reg, imm: i32 },
    Bgeu { rs1: Reg, rs2: Reg, imm: i32 },

    /// Opcode: LOAD
    Lb  { rd: Reg, rs1: Reg, imm: i32 },
    Lh  { rd: Reg, rs1: Reg, imm: i32 },
    Lw  { rd: Reg, rs1: Reg, imm: i32 },
    Lbu { rd: Reg, rs1: Reg, imm: i32 },
    Lhu { rd: Reg, rs1: Reg, imm: i32 },
    Lwu { rd: Reg, rs1: Reg, imm: i32 },
    Ld  { rd: Reg, rs1: Reg, imm: i32 },

    /// Opcode: STORE
    Sb { rs1: Reg, rs2: Reg, imm: i32 },
    Sh { rs1: Reg, rs2: Reg, imm: i32 },
    Sw { rs1: Reg, rs2: Reg, imm: i32 },
    Sd { rs1: Reg, rs2: Reg, imm: i32 },

    /// Opcode: OP-IMM
    Addi  { rd: Reg, rs1: Reg, imm: i32 },
    Slti  { rd: Reg, rs1: Reg, imm: i32 },
    Sltiu { rd: Reg, rs1: Reg, imm: i32 },
    Xori  { rd: Reg, rs1: Reg, imm: i32 },
    Ori   { rd: Reg, rs1: Reg, imm: i32 },
    Andi  { rd: Reg, rs1: Reg, imm: i32 },
    Slli  { rd: Reg, rs1: Reg, shamt: i32 },
    Srli  { rd: Reg, rs1: Reg, shamt: i32 },
    Srai  { rd: Reg, rs1: Reg, shamt: i32 },

    /// Opcode: OP-IMM-32
    Addiw { rd: Reg, rs1: Reg, imm: i32 },
    Slliw { rd: Reg, rs1: Reg, shamt: i32 },
    Srliw { rd: Reg, rs1: Reg, shamt: i32 },
    Sraiw { rd: Reg, rs1: Reg, shamt: i32 },

    /// Opcode: OP
    Add  { rd: Reg, rs1: Reg, rs2: Reg },
    Sub  { rd: Reg, rs1: Reg, rs2: Reg },
    Sll  { rd: Reg, rs1: Reg, rs2: Reg },
    Slt  { rd: Reg, rs1: Reg, rs2: Reg },
    Sltu { rd: Reg, rs1: Reg, rs2: Reg },
    Xor  { rd: Reg, rs1: Reg, rs2: Reg },
    Srl  { rd: Reg, rs1: Reg, rs2: Reg },
    Sra  { rd: Reg, rs1: Reg, rs2: Reg },
    Or   { rd: Reg, rs1: Reg, rs2: Reg },
    And  { rd: Reg, rs1: Reg, rs2: Reg },

    /// Opcode: OP-32
    Addw { rd: Reg, rs1: Reg, rs2: Reg },
    Subw { rd: Reg, rs1: Reg, rs2: Reg },
    Sllw { rd: Reg, rs1: Reg, rs2: Reg },
    Srlw { rd: Reg, rs1: Reg, rs2: Reg },
    Sraw { rd: Reg, rs1: Reg, rs2: Reg },

    /// Opcode: MISC-MEM
    Fence {}, // TODO(patrik): Fill in

    /// Opcode: SYSTEM
    Ecall {}, // TODO(patrik): Fill in
    Ebreak {}, // TODO(patrik): Fill in
}

impl Instruction {
    pub fn decode(inst: u32) -> Result<Self> {
        let opcode = inst & 0x7f;
        let opcode = Opcode::try_from(opcode)?;
        println!("Opcode: {:?}", opcode);

        return match opcode {
            Opcode::Lui => panic!("Not implemented: {:?}", opcode),
            Opcode::Auipc => panic!("Not implemented: {:?}", opcode),
            Opcode::Jal => {
                let data = JType::from(inst);
                Ok(Self::Jal { rd: data.rd, imm: data.imm })
            }
            Opcode::Jalr => {
                let data = IType::from(inst);
                Ok(Self::Jalr { rd: data.rd, rs1: data.rs1, imm: data.imm })
            },

            Opcode::Branch => panic!("Not implemented: {:?}", opcode),
            Opcode::Load => panic!("Not implemented: {:?}", opcode),
            Opcode::Store => panic!("Not implemented: {:?}", opcode),
            Opcode::OpImm => panic!("Not implemented: {:?}", opcode),
            Opcode::OpImm32 => panic!("Not implemented: {:?}", opcode),
            Opcode::Op => panic!("Not implemented: {:?}", opcode),
            Opcode::Op32 => panic!("Not implemented: {:?}", opcode),
            Opcode::MiscMem => panic!("Not implemented: {:?}", opcode),
            Opcode::System => panic!("Not implemented: {:?}", opcode),
        };
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
