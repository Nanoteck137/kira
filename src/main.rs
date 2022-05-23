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

    let res = elf::Elf::parse(&file_data).unwrap();
    println!("Elf: {:#?}", res);
}