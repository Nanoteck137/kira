use std::path::Path;
use std::fs::File;
use std::io::Read;

mod elf;

fn read_file_to_vec<P>(path: P) -> Vec<u8>
    where P: AsRef<Path>
{
    let mut file = File::open(path).unwrap();

    let mut result = Vec::new();
    file.read_to_end(&mut result).unwrap();

    result
}

fn main() {
    let path = "/opt/riscv/target/share/riscv-tests/isa/rv64ui-v-add";
    let file_data = read_file_to_vec(path);
    println!("Data: {}", file_data.len());

    let e = elf::Elf::parse(&file_data).unwrap();
    println!("Elf: {:#?}", e);

    let mut memory = vec![0; 100 * 1024 * 1024];
    const MEMORY_OFFSET: u64 = 0x80000000;

    let mut write_memory = |addr: u64, val: u8| {
        let memory_addr = addr.wrapping_sub(MEMORY_OFFSET);
        // println!("Write Memory: {:#x} -> {:#x}", memory_addr, val);
        memory[memory_addr as usize] = val;
    };

    for program_header in e.program_header_iter() {
        if program_header.typ() == elf::ProgramHeaderTyp::Load {
            let data = e.program_header_data(program_header)
                .expect("Failed to get program header data");
            println!("{:#x?}: {:#x}", program_header, data.len());

            for index in 0..data.len() {
                // println!("Writing: {}");
                // memory[index] = ;
                write_memory(program_header.vaddr() + index as u64, data[index]);
            }
        }
    }
}
