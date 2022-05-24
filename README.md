# kira

| Instruction | Opcode           |
| ----------- | ---------------- |
| LUI         | 0b0110111 (0x)   |
| AUIPC       | 0b0010111 (0x)   |
| JAL         | 0b1101111 (0x)   |
| JALR        | 0b1100111 (0x)   |
| BEQ         | 0b1100011 (0x)   |
| BNE         | 0b1100011 (0x)   |
| BLT         | 0b1100011 (0x)   |
| BGE         | 0b1100011 (0x)   |
| BLTU        | 0b1100011 (0x)   |
| BGEU        | 0b1100011 (0x)   |
| LB          | 0b0000011 (0x)   |
| LH          | 0b0000011 (0x)   |
| LW          | 0b0000011 (0x)   |
| LBU         | 0b0000011 (0x)   |
| LHU         | 0b0000011 (0x)   |
| SB          | 0b0100011 (0x)   |
| SH          | 0b0100011 (0x)   |
| SW          | 0b0100011 (0x)   |
| ADDI        | 0b0010011 (0x)   |
| SLTI        | 0b0010011 (0x)   |
| SLTIU       | 0b0010011 (0x)   |
| XORI        | 0b0010011 (0x)   |
| ORI         | 0b0010011 (0x)   |
| ANDI        | 0b0010011 (0x)   |
| SLLI        | 0b0010011 (0x)   |
| SRLI        | 0b0010011 (0x)   |
| SRAI        | 0b0010011 (0x)   |
| ADD         | 0b0110011 (0x)   |
| SUB         | 0b0110011 (0x)   |
| SLL         | 0b0110011 (0x)   |
| SLT         | 0b0110011 (0x)   |
| SLTU        | 0b0110011 (0x)   |
| XOR         | 0b0110011 (0x)   |
| SRL         | 0b0110011 (0x)   |
| SRA         | 0b0110011 (0x)   |
| OR          | 0b0110011 (0x)   |
| AND         | 0b0110011 (0x)   |
| FENCE       | 0b0001111 (0x)   |
| ECALL       | 0b1110011 (0x)   |
| EBREAK      | 0b1110011 (0x)   |
| LWU         | 0b0000011 (0x)   |
| LD          | 0b0000011 (0x)   |
| SD          | 0b0100011 (0x)   |
| SLLI        | 0b0010011 (0x)   |
| SRLI        | 0b0010011 (0x)   |
| SRAI        | 0b0010011 (0x)   |
| ADDIW       | 0b0011011 (0x)   |
| SLLIW       | 0b0011011 (0x)   |
| SRLIW       | 0b0011011 (0x)   |
| SRAIW       | 0b0011011 (0x)   |
| ADDW        | 0b0111011 (0x)   |
| SUBW        | 0b0111011 (0x)   |
| SLLW        | 0b0111011 (0x)   |
| SRLW        | 0b0111011 (0x)   |
| SRAW        | 0b0111011 (0x)   |

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