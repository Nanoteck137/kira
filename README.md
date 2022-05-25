# kira

# Types

| Name | 31-25  | 24-20 | 19-15 | 12-14  | 7-11 | 6-0    |
| ---- | ------ | ----- | ----- | ------ | ---- | ------ |
| R    | funct7 | rs2   | rs1   | funct3 | rd   | opcode |

---

| Name | 31-20     | 19-15 | 14-12  | 11-7 | 6-0    |
| ---- | --------- | ----- | ------ | ---- | ------ |
| I    | imm[11:0] | rs1   | funct3 | rd   | opcode |

---

| Name | 31-25     | 24-20 | 19-15 | 14-12  | 11-7     | 6-0    |
| ---- | --------- | ----- | ----- | ------ | -------- | ------ |
| S    | imm[11:5] | rs2   | rs1   | funct3 | imm[4:0] | opcode |

---

| Name | 31-25         | 24-20 | 19-15 | 14-12  | 11-7        | 6-0    |
| ---- | ------------- | ----- | ----- | ------ | ----------- | ------ |
| B    | imm[12\|10:5] | rs2   | rs1   | funct3 | imm[4:1|11] | opcode |

---

| Name | 31-12      | 11-7 | 6-0    |
| ---- | ---------- | ---- | ------ |
| U    | imm[31:12] | rd   | opcode |

| Name | 31-12                    | 11-7 | 6-0    |
| ---- | ------------------------ | ---- | ------ |
| J    | imm[20\|10:1\|11\|19:12] | rd   | opcode |

## LUI - 0b0110111
| Name | Type | Funct3 | Funct7  |
| ---- | ---- | ------ | ------- |
| LUI  |      | n/a    | n/a     |

## AUIPC - 0b0010111
| Name  | Funct3 | Funct7  |
| ----- | ------ | ------- |
| AUIPC | n/a    | n/a     |

## JAL - 0b1101111
| Name | Funct3 | Funct7  |
| ---- | ------ | ------- |
| JAL  | n/a    | n/a     |

## JALR - 0b1100111
| Name | Funct3 | Funct7  |
| ---- | ------ | ------- |
| JALR | 000    | n/a     |

## BRANCH - 0b1100011
| Name | Funct3 | Funct7  |
| ---- | ------ | ------- |
| BEQ  | 000    | n/a     |
| BNE  | 001    | n/a     |
| BLT  | 100    | n/a     |
| BGE  | 101    | n/a     |
| BLTU | 110    | n/a     |
| BGEU | 111    | n/a     |

## LOAD - 0b0000011
| Name | Funct3 | Funct7  |
| ---- | ------ | ------- |
| LB   | 000    | n/a     |
| LH   | 001    | n/a     |
| LW   | 010    | n/a     |
| LBU  | 100    | n/a     |
| LHU  | 101    | n/a     |
| LWU  | 110    | n/a     |
| LD   | 011    | n/a     |

## STORE - 0b0100011
| Name | Funct3 | Funct7  |
| ---- | ------ | ------- |
| SB   | 000    | n/a     |
| SH   | 001    | n/a     |
| SW   | 010    | n/a     |
| SD   | 011    | n/a     |

## OP-IMM - 0b0010011
| Name  | Funct3 | Funct7  |
| ----- | ------ | ------- |
| ADDI  | 000    | n/a     |
| SLTI  | 010    | n/a     |
| SLTIU | 011    | n/a     |
| XORI  | 100    | n/a     |
| ORI   | 110    | n/a     |
| ANDI  | 111    | n/a     |
| SLLI  | 001    | 0000000 |
| SRLI  | 101    | 0000000 |
| SRAI  | 101    | 0100000 |

## OP-IMM-32 - 0b0011011
| Name  | Funct3 | Funct7  |
| ----- | ------ | ------- |
| ADDIW | 000    | n/a     |
| SLLIW | 001    | 0000000 |
| SRLIW | 101    | 0000000 |
| SRAIW | 101    | 0100000 |

## OP - 0b0110011
| Name | Funct3 | Funct7  |
| ---- | ------ | ------- |
| ADD  | 000    | 0000000 |
| SUB  | 000    | 0100000 |
| SLL  | 001    | 0000000 |
| SLT  | 010    | 0000000 |
| SLTU | 011    | 0000000 |
| XOR  | 100    | 0000000 |
| SRL  | 101    | 0000000 |
| SRA  | 101    | 0100000 |
| OR   | 110    | 0000000 |
| AND  | 111    | 0000000 |

## OP-32 - 0b0111011
| Name | Funct3 | Funct7  |
| ---- | ------ | ------- |
| ADDW | 000    | 0000000 |
| SUBW | 000    | 0100000 |
| SLLW | 001    | 0000000 |
| SRLW | 101    | 0000000 |
| SRAW | 101    | 0100000 |

## MISC-MEM - 0b0001111
| Name  | Funct3 | Funct7  |
| ----- | ------ | ------- |
| FENCE | 000    | n/a     |

## SYSTEM - 0b1110011
| Name   | Funct3 | Funct7  |
| ------ | ------ | ------- |
| ECALL  | 000    | n/a     |
| EBREAK | 000    | n/a     |


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
