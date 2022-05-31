//! Module to handle CPU instructions

use super::Reg;

#[derive(Debug)]
pub enum Error {
    UnknownOpcode(u32),
    UnknownInstruction(Opcode, u32),
    Test
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
    Ecall,
    Ebreak,
    Sret,
    Mret,
    Csrrw { rd: Reg, rs1: Reg, csr: u16 },
    Csrrs { rd: Reg, rs1: Reg, csr: u16 },
    Csrrc { rd: Reg, rs1: Reg, csr: u16 },
    Csrrwi { rd: Reg, uimm: u32, csr: u16 },
    Csrrsi { rd: Reg, uimm: u32, csr: u16 },
    Csrrci { rd: Reg, uimm: u32, csr: u16 },
}

impl Instruction {
    pub fn decode(inst: u32) -> Result<Self> {
        let opcode = inst & 0x7f;
        let opcode = Opcode::try_from(opcode)?;
        println!("Opcode: {:?}", opcode);

        return match opcode {
            Opcode::Lui => {
                let data = UType::from(inst);
                Ok(Self::Lui { rd: data.rd, imm: data.imm })
            },

            Opcode::Auipc => {
                let data = UType::from(inst);
                Ok(Self::Auipc { rd: data.rd, imm: data.imm })
            },

            Opcode::Jal => {
                let data = JType::from(inst);
                Ok(Self::Jal { rd: data.rd, imm: data.imm })
            },

            Opcode::Jalr => {
                let data = IType::from(inst);
                Ok(Self::Jalr { rd: data.rd, rs1: data.rs1, imm: data.imm })
            },

            Opcode::Branch => Self::decode_branch(inst),
            Opcode::Load => Self::decode_load(inst),
            Opcode::Store => Self::decode_store(inst),
            Opcode::OpImm => Self::decode_op_imm(inst),
            Opcode::OpImm32 => Self::decode_op_imm_32(inst),
            Opcode::Op => Self::decode_op(inst),
            Opcode::Op32 => Self::decode_op_32(inst),
            Opcode::MiscMem => Self::decode_misc_mem(inst),
            Opcode::System => Self::decode_system(inst),
        };
    }

    fn decode_branch(inst: u32) -> Result<Self> {
        let data = BType::from(inst);

        let rs1 = data.rs1;
        let rs2 = data.rs2;
        let imm = data.imm;

        return match data.funct3 {
            0b000 => Ok(Self::Beq  { rs1, rs2, imm }),
            0b001 => Ok(Self::Bne  { rs1, rs2, imm }),
            0b100 => Ok(Self::Blt  { rs1, rs2, imm }),
            0b101 => Ok(Self::Bge  { rs1, rs2, imm }),
            0b110 => Ok(Self::Bltu { rs1, rs2, imm }),
            0b111 => Ok(Self::Bgeu { rs1, rs2, imm }),

            _ => Err(Error::UnknownInstruction(Opcode::Branch, inst)),
        };
    }

    fn decode_load(inst: u32) -> Result<Self> {
        let data = IType::from(inst);

        let rd = data.rd;
        let rs1 = data.rs1;
        let imm = data.imm;

        return match data.funct3 {
            0b000 => Ok(Self::Lb  { rd, rs1, imm }),
            0b001 => Ok(Self::Lh  { rd, rs1, imm }),
            0b010 => Ok(Self::Lw  { rd, rs1, imm }),
            0b100 => Ok(Self::Lbu { rd, rs1, imm }),
            0b101 => Ok(Self::Lhu { rd, rs1, imm }),
            0b110 => Ok(Self::Lwu { rd, rs1, imm }),
            0b011 => Ok(Self::Ld  { rd, rs1, imm }),

            _ => Err(Error::UnknownInstruction(Opcode::Load, inst)),
        };
    }

    fn decode_store(inst: u32) -> Result<Self> {
        let data = SType::from(inst);

        let rs1 = data.rs1;
        let rs2 = data.rs2;
        let imm = data.imm;

        return match data.funct3 {
            0b000 => Ok(Self::Sb { rs1, rs2, imm }),
            0b001 => Ok(Self::Sh { rs1, rs2, imm }),
            0b010 => Ok(Self::Sw { rs1, rs2, imm }),
            0b011 => Ok(Self::Sd { rs1, rs2, imm }),

            _ => Err(Error::UnknownInstruction(Opcode::Store, inst)),
        };
    }

    fn decode_op_imm(inst: u32) -> Result<Self> {
        let data = IType::from(inst);

        let rd = data.rd;
        let rs1 = data.rs1;
        let imm = data.imm;

        let shamt = imm & 0x3f;
        let mode = (imm >> 6) & 0x3f;

        return match data.funct3 {
            0b000 => Ok(Self::Addi  { rd, rs1, imm }),
            0b010 => Ok(Self::Slti  { rd, rs1, imm }),
            0b011 => Ok(Self::Sltiu { rd, rs1, imm }),
            0b100 => Ok(Self::Xori  { rd, rs1, imm }),
            0b110 => Ok(Self::Ori   { rd, rs1, imm }),
            0b111 => Ok(Self::Andi  { rd, rs1, imm }),
            0b001 => Ok(Self::Slli  { rd, rs1, shamt }),
            0b101 => {
                match mode {
                    0b0000000 => Ok(Self::Srli  { rd, rs1, shamt }),
                    0b0100000 => Ok(Self::Srai  { rd, rs1, shamt }),

                    // TODO(patrik): Diffrent error?
                    _ => Err(Error::UnknownInstruction(Opcode::OpImm, inst)),
                }
            },

            _ => Err(Error::UnknownInstruction(Opcode::OpImm, inst)),
        };
    }

    fn decode_op_imm_32(inst: u32) -> Result<Self> {
        let data = IType::from(inst);

        let rd = data.rd;
        let rs1 = data.rs1;
        let imm = data.imm;

        let shamt = imm & 0x3f;
        let mode = (imm >> 6) & 0x3f;

        return match data.funct3 {
            0b000 => Ok(Self::Addiw { rd, rs1, imm }),
            0b001 => Ok(Self::Slliw { rd, rs1, shamt }),
            0b101 => {
                match mode {
                    0b0000000 => Ok(Self::Srliw { rd, rs1, shamt }),
                    0b0100000 => Ok(Self::Sraiw { rd, rs1, shamt }),

                    // TODO(patrik): Diffrent error?
                    _ => Err(Error::UnknownInstruction(Opcode::OpImm32, inst)),
                }
            },

            _ => Err(Error::UnknownInstruction(Opcode::OpImm32, inst)),
        };
    }

    fn decode_op(inst: u32) -> Result<Self> {
        let data = RType::from(inst);

        let rd = data.rd;
        let rs1 = data.rs1;
        let rs2 = data.rs2;

        return match (data.funct3, data.funct7) {
            (0b000, 0b0000000) => Ok(Self::Add  { rd, rs1, rs2 }),
            (0b000, 0b0100000) => Ok(Self::Sub  { rd, rs1, rs2 }),
            (0b001, 0b0000000) => Ok(Self::Sll  { rd, rs1, rs2 }),
            (0b010, 0b0000000) => Ok(Self::Slt  { rd, rs1, rs2 }),
            (0b011, 0b0000000) => Ok(Self::Sltu { rd, rs1, rs2 }),
            (0b100, 0b0000000) => Ok(Self::Xor  { rd, rs1, rs2 }),
            (0b101, 0b0000000) => Ok(Self::Srl  { rd, rs1, rs2 }),
            (0b101, 0b0100000) => Ok(Self::Sra  { rd, rs1, rs2 }),
            (0b110, 0b0000000) => Ok(Self::Or   { rd, rs1, rs2 }),
            (0b111, 0b0000000) => Ok(Self::And  { rd, rs1, rs2 }),

            // TODO(patrik): Diffrent error?
            _ => Err(Error::UnknownInstruction(Opcode::Op, inst)),
        };
    }

    fn decode_op_32(inst: u32) -> Result<Self> {
        let data = RType::from(inst);

        let rd = data.rd;
        let rs1 = data.rs1;
        let rs2 = data.rs2;

        return match (data.funct3, data.funct7) {
            (0b000, 0b0000000) => Ok(Self::Addw { rd, rs1, rs2 }),
            (0b000, 0b0100000) => Ok(Self::Subw { rd, rs1, rs2 }),
            (0b001, 0b0000000) => Ok(Self::Sllw { rd, rs1, rs2 }),
            (0b101, 0b0000000) => Ok(Self::Srlw { rd, rs1, rs2 }),
            (0b101, 0b0100000) => Ok(Self::Sraw { rd, rs1, rs2 }),

            // TODO(patrik): Diffrent error?
            _ => Err(Error::UnknownInstruction(Opcode::Op32, inst)),
        };
    }

    fn decode_misc_mem(inst: u32) -> Result<Self> {
        let data = IType::from(inst);
        return match data.funct3 {
            0b000 => Ok(Self::Fence { }),

            _ => Err(Error::UnknownInstruction(Opcode::MiscMem, inst)),
        };
    }

    fn decode_system(inst: u32) -> Result<Self> {
        let data = IType::from(inst);

        let rd = data.rd;
        let rs1 = data.rs1;

        let imm = data.imm;
        let uimm = (inst >> 15) & 0x1f;
        let csr = (imm & 0xfff) as u16;

        return match data.funct3 {
            0b000 => {
                let rdata = RType::from(inst);
                let funct5 = (inst >> 20) & 0x1f;

                match (funct5, rdata.funct7) {
                    (0b00000, 0b0000000) => Ok(Self::Ecall {}),
                    (0b00001, 0b0000000) => Ok(Self::Ebreak {}),
                    (0b00010, 0b0001000) => Ok(Self::Sret {}),
                    (0b00010, 0b0011000) => Ok(Self::Mret {}),

                    _ => Err(Error::UnknownInstruction(Opcode::System, inst)),
                }
            },

            0b001 => Ok(Self::Csrrw { rd, rs1, csr }),
            0b010 => Ok(Self::Csrrs { rd, rs1, csr }),
            0b011 => Ok(Self::Csrrc { rd, rs1, csr }),
            0b101 => Ok(Self::Csrrwi { rd, uimm, csr }),
            0b110 => Ok(Self::Csrrsi { rd, uimm, csr }),
            0b111 => Ok(Self::Csrrci { rd, uimm, csr }),

            _ => Err(Error::UnknownInstruction(Opcode::System, inst)),
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
