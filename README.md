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
| Name | Type |
| ---- | ---- |
| LUI  | U    |

## AUIPC - 0b0010111
| Name  | Type |
| ----- | ---- |
| AUIPC | U    |

## JAL - 0b1101111
| Name | Type | Funct3 | Funct7  |
| ---- | ---- | ------ | ------- |
| JAL  | J    | n/a    | n/a     |

## JALR - 0b1100111
| Name | Type | Funct3 | Funct7  |
| ---- | ---- | ------ | ------- |
| JALR | I    | 000    | n/a     |

## BRANCH - 0b1100011
| Name | Type | Funct3 | Funct7  |
| ---- | ---- | ------ | ------- |
| BEQ  | B    | 000    | n/a     |
| BNE  | B    | 001    | n/a     |
| BLT  | B    | 100    | n/a     |
| BGE  | B    | 101    | n/a     |
| BLTU | B    | 110    | n/a     |
| BGEU | B    | 111    | n/a     |

## LOAD - 0b0000011
| Name | Type | Funct3 | Funct7  |
| ---- | ---- | ------ | ------- |
| LB   | I    | 000    | n/a     |
| LH   | I    | 001    | n/a     |
| LW   | I    | 010    | n/a     |
| LBU  | I    | 100    | n/a     |
| LHU  | I    | 101    | n/a     |
| LWU  | I    | 110    | n/a     |
| LD   | I    | 011    | n/a     |

## STORE - 0b0100011
| Name | Type | Funct3 | Funct7  |
| ---- | ---- | ------ | ------- |
| SB   | S    | 000    | n/a     |
| SH   | S    | 001    | n/a     |
| SW   | S    | 010    | n/a     |
| SD   | S    | 011    | n/a     |

## OP-IMM - 0b0010011
| Name  | Type | Funct3 | Funct7  |
| ----- | ---- | ------ | ------- |
| ADDI  | I    | 000    | n/a     |
| SLTI  | I    | 010    | n/a     |
| SLTIU | I    | 011    | n/a     |
| XORI  | I    | 100    | n/a     |
| ORI   | I    | 110    | n/a     |
| ANDI  | I    | 111    | n/a     |
| SLLI  | I*   | 001    | 0000000 |
| SRLI  | I*   | 101    | 0000000 |
| SRAI  | I*   | 101    | 0100000 |

## OP-IMM-32 - 0b0011011
| Name  | Type | Funct3 | Funct7  |
| ----- | ---- | ------ | ------- |
| ADDIW | I    | 000    | n/a     |
| SLLIW | I*   | 001    | 0000000 |
| SRLIW | I*   | 101    | 0000000 |
| SRAIW | I*   | 101    | 0100000 |

## OP - 0b0110011
| Name | Type | Funct3 | Funct7  |
| ---- | ---- | ------ | ------- |
| ADD  | R    | 000    | 0000000 |
| SUB  | R    | 000    | 0100000 |
| SLL  | R    | 001    | 0000000 |
| SLT  | R    | 010    | 0000000 |
| SLTU | R    | 011    | 0000000 |
| XOR  | R    | 100    | 0000000 |
| SRL  | R    | 101    | 0000000 |
| SRA  | R    | 101    | 0100000 |
| OR   | R    | 110    | 0000000 |
| AND  | R    | 111    | 0000000 |

## OP-32 - 0b0111011
| Name | Type | Funct3 | Funct7  |
| ---- | ---- | ------ | ------- |
| ADDW | R    | 000    | 0000000 |
| SUBW | R    | 000    | 0100000 |
| SLLW | R    | 001    | 0000000 |
| SRLW | R    | 101    | 0000000 |
| SRAW | R    | 101    | 0100000 |

## MISC-MEM - 0b000111
| Name  | Type | Funct3 |
| ----- | ---- | ------ |
| FENCE | I    | 000    |

## SYSTEM - 0b1110011
| Name   | Type | Funct3 | Funct7  |
| ------ | ---- | ------ | ------- |
| ECALL  | *    | 000    | n/a     |
| EBREAK | *    | 000    | n/a     |


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
