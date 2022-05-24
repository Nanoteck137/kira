# kira

# LUI
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| LUI         | 0b0110111 (0x)   | n/a    | n/a     |

# AUIPC
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| AUIPC       | 0b0010111 (0x)   | n/a    | n/a     |

# JAL
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| JAL         | 0b1101111 (0x)   | n/a    | n/a     |

# JALR
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| JALR        | 0b1100111 (0x)   | 000    | n/a     |

# BRANCH
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| BEQ         | 0b1100011 (0x)   | 000    | n/a     |
| BNE         | 0b1100011 (0x)   | 001    | n/a     |
| BLT         | 0b1100011 (0x)   | 100    | n/a     |
| BGE         | 0b1100011 (0x)   | 101    | n/a     |
| BLTU        | 0b1100011 (0x)   | 110    | n/a     |
| BGEU        | 0b1100011 (0x)   | 111    | n/a     |

# LOAD
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| LB          | 0b0000011 (0x)   | 000    | n/a     |
| LH          | 0b0000011 (0x)   | 001    | n/a     |
| LW          | 0b0000011 (0x)   | 010    | n/a     |
| LBU         | 0b0000011 (0x)   | 100    | n/a     |
| LHU         | 0b0000011 (0x)   | 101    | n/a     |
| LWU         | 0b0000011 (0x)   | 110    | n/a     |
| LD          | 0b0000011 (0x)   | 011    | n/a     |

# STORE
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| SB          | 0b0100011 (0x)   | 000    | n/a     |
| SH          | 0b0100011 (0x)   | 001    | n/a     |
| SW          | 0b0100011 (0x)   | 010    | n/a     |
| SD          | 0b0100011 (0x)   | 011    | n/a     |

# OP-IMM
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| ADDI        | 0b0010011 (0x)   | 000    | n/a     |
| SLTI        | 0b0010011 (0x)   | 010    | n/a     |
| SLTIU       | 0b0010011 (0x)   | 011    | n/a     |
| XORI        | 0b0010011 (0x)   | 100    | n/a     |
| ORI         | 0b0010011 (0x)   | 110    | n/a     |
| ANDI        | 0b0010011 (0x)   | 111    | n/a     |
| SLLI        | 0b0010011 (0x)   | 001    | 0000000 |
| SRLI        | 0b0010011 (0x)   | 101    | 0000000 |
| SRAI        | 0b0010011 (0x)   | 101    | 0100000 |

# OP-IMM-32
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| ADDIW       | 0b0011011 (0x)   | 000    | n/a     |
| SLLIW       | 0b0011011 (0x)   | 001    | 0000000 |
| SRLIW       | 0b0011011 (0x)   | 101    | 0000000 |
| SRAIW       | 0b0011011 (0x)   | 101    | 0100000 |

# OP
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| ADD         | 0b0110011 (0x)   | 000    | 0000000 |
| SUB         | 0b0110011 (0x)   | 000    | 0100000 |
| SLL         | 0b0110011 (0x)   | 001    | 0000000 |
| SLT         | 0b0110011 (0x)   | 010    | 0000000 |
| SLTU        | 0b0110011 (0x)   | 011    | 0000000 |
| XOR         | 0b0110011 (0x)   | 100    | 0000000 |
| SRL         | 0b0110011 (0x)   | 101    | 0000000 |
| SRA         | 0b0110011 (0x)   | 101    | 0100000 |
| OR          | 0b0110011 (0x)   | 110    | 0000000 |
| AND         | 0b0110011 (0x)   | 111    | 0000000 |

# OP-32
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| ADDW        | 0b0111011 (0x)   | 000    | 0000000 |
| SUBW        | 0b0111011 (0x)   | 000    | 0100000 |
| SLLW        | 0b0111011 (0x)   | 001    | 0000000 |
| SRLW        | 0b0111011 (0x)   | 101    | 0000000 |
| SRAW        | 0b0111011 (0x)   | 101    | 0100000 |

# MISC-MEM
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| FENCE       | 0b0001111 (0x)   | 000    | n/a     |

# SYSTEM
| Opcode Name | Opcode           | Funct3 | Funct7  |
| ----------- | ---------------- | ------ | ------- |
| ECALL       | 0b1110011 (0x)   | 000    | n/a     |
| EBREAK      | 0b1110011 (0x)   | 000    | n/a     |


---

* RV32I Base Instruction Set
0110111 LUI
0010111 AUIPC
1101111 JAL
1100111 JALR
1100011 BEQ
1100011 BNE
1100011 BLT
1100011 BGE
1100011 BLTU
1100011 BGEU
0000011 LB
0000011 LH
0000011 LW
0000011 LBU
0000011 LHU
0100011 SB
0100011 SH
0100011 SW
0010011 ADDI
0010011 SLTI
0010011 SLTIU
0010011 XORI
0010011 ORI
0010011 ANDI
0010011 SLLI
0010011 SRLI
0010011 SRAI
0110011 ADD
0110011 SUB
0110011 SLL
0110011 SLT
0110011 SLTU
0110011 XOR
0110011 SRL
0110011 SRA
0110011 OR
0110011 AND
0001111 FENCE
1110011 ECALL
1110011 EBREAK

RV64I Base Instruction Set (in addition to RV32I)
0000011 LWU
0000011 LD
0100011 SD
0010011 SLLI
0010011 SRLI
0010011 SRAI
0011011 ADDIW
0011011 SLLIW
0011011 SRLIW
0011011 SRAIW
0111011 ADDW
0111011 SUBW
0111011 SLLW
0111011 SRLW
0111011 SRAW