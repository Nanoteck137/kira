#![allow(dead_code)]

use std::path::{ Path, PathBuf };
use std::fs::File;
use std::io::Read;

use memory::{ TestingMemory, TestingMmu, Mmu };
use cpu::{ SimpleHart, Hart, Reg };

mod elf;
mod memory;
mod cpu;

fn read_file_to_vec<P>(path: P) -> Vec<u8>
    where P: AsRef<Path>
{
    let mut file = File::open(path).unwrap();

    let mut result = Vec::new();
    file.read_to_end(&mut result).unwrap();

    result
}

fn run_test(test: &str) -> Result<(), u32> {
    let mut path = PathBuf::from("/opt/riscv/target/share/riscv-tests/isa/"); 
    path.push(test);
    let file_data = read_file_to_vec(path);

    let e = elf::Elf::parse(&file_data).unwrap();
    // println!("Elf: {:#?}", e);

    let memory = TestingMemory::new(100 * 1024 * 1024);
    let mut mmu = TestingMmu::new(memory);

    for program_header in e.program_header_iter() {
        if program_header.typ() == elf::ProgramHeaderTyp::Load {
            let data = e.program_header_data(program_header)
                .expect("Failed to get program header data");
            // println!("{:#x?}: {:#x}", program_header, data.len());

            for index in 0..data.len() {
                let addr = program_header.vaddr() + index as u64;
                let value = data[index];
                mmu.write_u8(addr, value);
            }
        }
    }

    let mut hart = SimpleHart::new(Box::new(mmu));
    hart.set_reg(Reg::Pc, e.entry());
    // hart.dump();

    loop {
        hart.step();

        let value = hart.mmu.read_u32(0x80001000);
        let success = (value & 0x1) == 1;
        let testnum = value >> 1;

        if success {
            if testnum == 0 {
                return Ok(());
            } 

            if testnum > 0 {
                return Err(testnum);
            }
        }
    }

}

fn main() {
    let tests = [
        "rv64ui-p-add",
        "rv64ui-p-addi",
        "rv64ui-p-addiw",
        "rv64ui-p-addw",
        "rv64ui-p-and",
        "rv64ui-p-andi",
        "rv64ui-p-auipc",
        "rv64ui-p-beq",
        "rv64ui-p-bge",
        "rv64ui-p-bgeu",
        "rv64ui-p-blt",
        "rv64ui-p-bltu",
        "rv64ui-p-bne",
        // "rv64ui-p-fence_i", BROKEN
        "rv64ui-p-jal",
        "rv64ui-p-jalr",
        "rv64ui-p-lb",
        "rv64ui-p-lbu",
        "rv64ui-p-ld",
        "rv64ui-p-lh",
        "rv64ui-p-lhu",
        "rv64ui-p-lui",
        "rv64ui-p-lw",
        "rv64ui-p-lwu",
        "rv64ui-p-or",
        "rv64ui-p-ori",
        "rv64ui-p-sb",
        "rv64ui-p-sd",
        "rv64ui-p-sh",
        "rv64ui-p-simple",
        "rv64ui-p-sll",
        "rv64ui-p-slli",
        "rv64ui-p-slliw",
        "rv64ui-p-sllw",
        "rv64ui-p-slt",
        "rv64ui-p-slti",
        "rv64ui-p-sltiu",
        "rv64ui-p-sltu",
        "rv64ui-p-sra",
        "rv64ui-p-srai",
        "rv64ui-p-sraiw",
        "rv64ui-p-sraw",
        "rv64ui-p-srl",
        "rv64ui-p-srli",
        "rv64ui-p-srliw",
        "rv64ui-p-srlw",
        "rv64ui-p-sub",
        "rv64ui-p-subw",
        "rv64ui-p-sw",
        "rv64ui-p-xor",
        "rv64ui-p-xori",
    ];

    for test in tests {
        print!("{}: ", test);
        let res = run_test(test);
        match res {
            Ok(_) => println!("Passed"),
            Err(num) => println!("Failed at test#{}", num),
        }
    }

}
