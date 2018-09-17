extern crate riscv_disasm;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use riscv_disasm::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let isa_name = &args[1];
    let filename = &args[2];
    let start_pc = u64::from_str_radix(&args[3], 16).unwrap();

    let isa = match isa_name.as_str() {
        "rv32" => rv_isa::rv32,
        "rv64" => rv_isa::rv64,
        "rv128" => rv_isa::rv128,
        _ => panic!("Invalid RISC-V ISA"),
    };

    let mut f = File::open(filename).expect("file not found");
    let mut buffer = Vec::<u8>::new();
    f.read_to_end(&mut buffer)
        .expect("cannot read file into buffer");

    for decoded in Disassembler::new(isa, &buffer, start_pc) {
        println!("{:08x} {}", decoded.pc, format_inst(32, &decoded));
    }
}
