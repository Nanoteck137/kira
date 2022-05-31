#![allow(dead_code)]

use std::path::Path;
use std::fs::File;
use std::io::Read;

use memory::{ Memory, Mmu };
use cpu::{ Hart, Reg };

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

fn main() {
    let path = "/opt/riscv/target/share/riscv-tests/isa/rv64ui-p-add";
    let file_data = read_file_to_vec(path);
    println!("Data: {}", file_data.len());

    let e = elf::Elf::parse(&file_data).unwrap();
    println!("Elf: {:#?}", e);

    let memory = Memory::new(100 * 1024 * 1024);
    let mut mmu = Mmu::new(memory);

    for program_header in e.program_header_iter() {
        if program_header.typ() == elf::ProgramHeaderTyp::Load {
            let data = e.program_header_data(program_header)
                .expect("Failed to get program header data");
            println!("{:#x?}: {:#x}", program_header, data.len());

            for index in 0..data.len() {
                let addr = program_header.vaddr() + index as u64;
                let value = data[index];
                mmu.write_u8(addr, value);
            }
        }
    }

    let mut hart = Hart::new(mmu);
    hart.set_reg(Reg::Pc, e.entry());
    hart.dump();

    loop {
        hart.step();
        // hart.dump();
    }
}
