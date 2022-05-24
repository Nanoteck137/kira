//! Module to handle CPU instructions

use super::Reg;

#[derive(Debug)]
pub enum Instruction {
    /// Opcode: 0b0010111
    Auipc { rd: Reg, imm: i32 },

    /// Opcode: 0b1101111
    Jal { rd: Reg, imm: i32 },

    /// Opcode: 0b1100111
    Jalr { rd: Reg, rs1: Reg, imm: i32 },

    /// Opcode: 0b0010011
    Addi  { rd: Reg, rs1: Reg, imm: i32 },
    Slti  { rd: Reg, rs1: Reg, imm: i32 },
    Sltiu { rd: Reg, rs1: Reg, imm: i32 },
    Xori  { rd: Reg, rs1: Reg, imm: i32 },
    Ori   { rd: Reg, rs1: Reg, imm: i32 },
    Andi  { rd: Reg, rs1: Reg, imm: i32 },
    Slli  { rd: Reg, rs1: Reg, shamt: i32 },
    Srli  { rd: Reg, rs1: Reg, shamt: i32 },
    Srai  { rd: Reg, rs1: Reg, shamt: i32 },


    /// Opcode: 0b0110011
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

    /// Opcode: 0b1110011
    Csrrw  { rd: Reg, rs1: Reg, csr: u16 },
    Csrrs  { rd: Reg, rs1: Reg, csr: u16 },
    Csrrc  { rd: Reg, rs1: Reg, csr: u16 },
    Csrrwi { rd: Reg, uimm: u32, csr: u16 },
    Csrrsi { rd: Reg, uimm: u32, csr: u16 },
    Csrrci { rd: Reg, uimm: u32, csr: u16 },
}

impl Instruction {
    pub fn decode(inst: u32) -> Self {
        let opcode = inst & 0x7f;
        let typ = OPCODE_TO_TYPE_LUT[opcode as usize];
        if let Some(typ) = typ {
            let test = JType::from(inst);

            return match typ {
                Type::R => Self::decode_r(opcode, inst),
                Type::I => Self::decode_i(opcode, inst),
                Type::S => panic!("S-Type not implemented"),
                Type::B => panic!("B-Type not implemented"),
                Type::U => Self::decode_u(opcode, inst),
                Type::J => Self::decode_j(opcode, inst),
            };
        } else {
            panic!("Unknown upcode type: 0b{:07b}", opcode);
        }
    }

    pub fn decode_r(opcode: u32, inst: u32) -> Self {
        let data = RType::from(inst);

        return match opcode {
            0b0110011 => {
                match data.funct3 {
                    0b000 => {
                        match data.funct7 {
                            0b0000000 => Instruction::Add { rd: data.rd, rs1: data.rs1, rs2: data.rs2 },
                            0b0100000 => Instruction::Sub { rd: data.rd, rs1: data.rs1, rs2: data.rs2 },
                            _ => panic!("Unknown 0b{:07b} funct3 0b{:03b} funct7: 0b{:07b}",
                                        opcode, data.funct3, data.funct7),
                        }
                    }
                    /*
                    0b001 => {},
                    0b010 => {},
                    0b011 => {},
                    0b100 => {},
                    0b101 => {},
                    0b110 => {},
                    0b111 => {},
                    */

                    _ => panic!("Unknown 0b{:07b} funct3 0b{:03b}",
                                opcode, data.funct3),
                }
/*
0000000 rs2 rs1 000 rd 0110011 ADD
0100000 rs2 rs1 000 rd 0110011 SUB
0000000 rs2 rs1 001 rd 0110011 SLL
0000000 rs2 rs1 010 rd 0110011 SLT
0000000 rs2 rs1 011 rd 0110011 SLTU
0000000 rs2 rs1 100 rd 0110011 XOR
0000000 rs2 rs1 101 rd 0110011 SRL
0100000 rs2 rs1 101 rd 0110011 SRA
0000000 rs2 rs1 110 rd 0110011 OR
0000000 rs2 rs1 111 rd 0110011 AND
*/
            },
            _ => panic!("Unknown R-Type Opcode: 0b{:07b}", opcode),
        };
    }

    pub fn decode_j(opcode: u32, inst: u32) -> Self {
        let data = JType::from(inst);

        return match opcode {
            0b1101111 => {
                // JAL
                Self::Jal { rd: data.rd, imm: data.imm }
            },

            _ => panic!("Unknown J-Type Opcode: 0b{:07b}", opcode),
        };
    }

    pub fn decode_i(opcode: u32, inst: u32) -> Self {
        let data = IType::from(inst);

        return match opcode {
            0b1100111 => Self::Jalr { rd: data.rd, rs1: data.rs1, imm: data.imm },

            0b0010011 => {
                let shamt = (data.imm >> 0) & 0x3f;
                let mode  = (data.imm >> 6) & 0x3f;
                match data.funct3 {
                    0b000 => Self::Addi  { rd: data.rd, rs1: data.rs1, imm: data.imm },
                    0b010 => Self::Slti  { rd: data.rd, rs1: data.rs1, imm: data.imm },
                    0b011 => Self::Sltiu { rd: data.rd, rs1: data.rs1, imm: data.imm },
                    0b100 => Self::Xori  { rd: data.rd, rs1: data.rs1, imm: data.imm },
                    0b110 => Self::Ori   { rd: data.rd, rs1: data.rs1, imm: data.imm },
                    0b111 => Self::Andi  { rd: data.rd, rs1: data.rs1, imm: data.imm },

                    0b001 => Self::Slli { rd: data.rd, rs1: data.rs1, shamt },
                    0b101 => {
                        match mode {
                            0b000000 => Self::Srli { rd: data.rd, rs1: data.rs1, shamt },
                            0b010000 => Self::Srai { rd: data.rd, rs1: data.rs1, shamt },
                            _ => panic!("Unknown 0b{:07b} funct3 0b{:03b} mode {:06b}",
                                        opcode, data.funct3, mode),
                        }
                    },

                    _ => panic!("Unknown 0b{:07b} funct3 0b{:03b}",
                                opcode, data.funct3),
                }
            },

            0b1110011 => {
                let csr = ((inst >> 20) & 0xfff) as u16;
                let uimm = ((inst >> 15) & 0x1f) as u32;
                match data.funct3 {
                    0b001 => Self::Csrrw  { rd: data.rd, rs1: data.rs1, csr },
                    0b010 => Self::Csrrs  { rd: data.rd, rs1: data.rs1, csr },
                    0b011 => Self::Csrrc  { rd: data.rd, rs1: data.rs1, csr },
                    0b101 => Self::Csrrwi { rd: data.rd, uimm, csr },
                    0b110 => Self::Csrrsi { rd: data.rd, uimm, csr },
                    0b111 => Self::Csrrci { rd: data.rd, uimm, csr },
                    _ => panic!("Unknown 0b{:07b} funct3 0b{:03b}",
                                opcode, data.funct3),
                }
            },

            _ => panic!("Unknown I-Type Opcode: 0b{:07b}", opcode),
        };
    }

    pub fn decode_u(opcode: u32, inst: u32) -> Self {
        let data = UType::from(inst);

        return match opcode {
            0b0010111 => Self::Auipc { rd: data.rd, imm: data.imm },
            _ => panic!("Unknown U-Type Opcode: 0b{:07b}", opcode),
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Type {
    R,
    I,
    S,
    B,
    U,
    J
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

static OPCODE_TO_TYPE_LUT: [Option<Type>; 128] = [
    None,          // 0b0000000
    None,          // 0b0000001
    None,          // 0b0000010
    None,          // 0b0000011
    None,          // 0b0000100
    None,          // 0b0000101
    None,          // 0b0000110
    None,          // 0b0000111
    None,          // 0b0001000
    None,          // 0b0001001
    None,          // 0b0001010
    None,          // 0b0001011
    None,          // 0b0001100
    None,          // 0b0001101
    None,          // 0b0001110
    None,          // 0b0001111
    None,          // 0b0010000
    None,          // 0b0010001
    None,          // 0b0010010
    Some(Type::I), // 0b0010011
    None,          // 0b0010100
    None,          // 0b0010101
    None,          // 0b0010110
    Some(Type::U), // 0b0010111
    None,          // 0b0011000
    None,          // 0b0011001
    None,          // 0b0011010
    None,          // 0b0011011
    None,          // 0b0011100
    None,          // 0b0011101
    None,          // 0b0011110
    None,          // 0b0011111
    None,          // 0b0100000
    None,          // 0b0100001
    None,          // 0b0100010
    None,          // 0b0100011
    None,          // 0b0100100
    None,          // 0b0100101
    None,          // 0b0100110
    None,          // 0b0100111
    None,          // 0b0101000
    None,          // 0b0101001
    None,          // 0b0101010
    None,          // 0b0101011
    None,          // 0b0101100
    None,          // 0b0101101
    None,          // 0b0101110
    None,          // 0b0101111
    None,          // 0b0110000
    None,          // 0b0110001
    None,          // 0b0110010
    Some(Type::R), // 0b0110011
    None,          // 0b0110100
    None,          // 0b0110101
    None,          // 0b0110110
    None,          // 0b0110111
    None,          // 0b0111000
    None,          // 0b0111001
    None,          // 0b0111010
    None,          // 0b0111011
    None,          // 0b0111100
    None,          // 0b0111101
    None,          // 0b0111110
    None,          // 0b0111111
    None,          // 0b1000000
    None,          // 0b1000001
    None,          // 0b1000010
    None,          // 0b1000011
    None,          // 0b1000100
    None,          // 0b1000101
    None,          // 0b1000110
    None,          // 0b1000111
    None,          // 0b1001000
    None,          // 0b1001001
    None,          // 0b1001010
    None,          // 0b1001011
    None,          // 0b1001100
    None,          // 0b1001101
    None,          // 0b1001110
    None,          // 0b1001111
    None,          // 0b1010000
    None,          // 0b1010001
    None,          // 0b1010010
    None,          // 0b1010011
    None,          // 0b1010100
    None,          // 0b1010101
    None,          // 0b1010110
    None,          // 0b1010111
    None,          // 0b1011000
    None,          // 0b1011001
    None,          // 0b1011010
    None,          // 0b1011011
    None,          // 0b1011100
    None,          // 0b1011101
    None,          // 0b1011110
    None,          // 0b1011111
    None,          // 0b1100000
    None,          // 0b1100001
    None,          // 0b1100010
    None,          // 0b1100011
    None,          // 0b1100100
    None,          // 0b1100101
    None,          // 0b1100110
    Some(Type::I), // 0b1100111
    None,          // 0b1101000
    None,          // 0b1101001
    None,          // 0b1101010
    None,          // 0b1101011
    None,          // 0b1101100
    None,          // 0b1101101
    None,          // 0b1101110
    Some(Type::J), // 0b1101111
    None,          // 0b1110000
    None,          // 0b1110001
    None,          // 0b1110010
    Some(Type::I), // 0b1110011
    None,          // 0b1110100
    None,          // 0b1110101
    None,          // 0b1110110
    None,          // 0b1110111
    None,          // 0b1111000
    None,          // 0b1111001
    None,          // 0b1111010
    None,          // 0b1111011
    None,          // 0b1111100
    None,          // 0b1111101
    None,          // 0b1111110
    None,          // 0b1111111
];
