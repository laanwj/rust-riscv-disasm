/* scan all instructions consisting of alphanumeric characters only,
 * print the legal ones
 *
 * usage: `instscan <16|32> <verbose>`
 *
 * (where *verbose* is 'true' or 'false')
 */
extern crate riscv_disasm;

use riscv_disasm::*;
use std::env;
use std::collections::HashMap;

const ALPHANUMERIC: &[u8] = &[
   0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
   0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a,
   0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bit {
    Unknown,
    Off,
    On,
    Mixed,
}

#[derive(Copy, Clone)]
struct BitStats {
    bit: [Bit; 64],
}

impl BitStats {
    fn new() -> BitStats {
        BitStats { bit: [Bit::Unknown; 64] }
    }

    fn to_string(self, len: usize) -> String {
        self.bit[0..len].iter().rev().map(
            |i| {
                match i {
                    Bit::Unknown => '?',
                    Bit::Off => '0',
                    Bit::On => '1',
                    Bit::Mixed => 'x',
                }
            }
        ).collect::<String>()
    }
}

fn add(stats: &mut HashMap<rv_op, BitStats>, op: rv_op, pattern: u64) {
    let bits = stats.entry(op).or_insert(BitStats::new());
    for b in 0..64 {
        if (pattern & (1<<b)) != 0 { // Bit is on
            bits.bit[b] = match bits.bit[b] {
                Bit::Unknown => Bit::On,
                Bit::Off => Bit::Mixed,
                Bit::On => Bit::On,
                Bit::Mixed => Bit::Mixed,
            }
        } else { // Bit is off
            bits.bit[b] = match bits.bit[b] {
                Bit::Unknown => Bit::Off,
                Bit::Off => Bit::Off,
                Bit::On => Bit::Mixed,
                Bit::Mixed => Bit::Mixed,
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let sz = args[1].parse::<usize>().expect("Invalid bit size");
    let verbose = args[2].parse::<bool>().expect("Invalid verbose flag");
    let mut stats: HashMap<rv_op, BitStats> = HashMap::new();
    match sz  {
        16 => {
            for a in ALPHANUMERIC {
                if (*a & 3)!=3 { /* 32 bit instruction */
                    for b in ALPHANUMERIC {
                        let inst = ((*b as u64)<<8) | *a as u64;
                        let d = decode_inst(rv_isa::rv32, 0, inst);
                        if d.op != rv_op::illegal {
                            add(&mut stats, d.op, inst);
                            if verbose {
                                println!("{}", format_inst(32, &d));
                            }
                        }
                    }
                }
            }
        }
        32 => {
            for a in ALPHANUMERIC {
                if (*a & 3)==3 { /* 32 bit instruction */
                    for b in ALPHANUMERIC {
                        for c in ALPHANUMERIC {
                            for d in ALPHANUMERIC {
                                let inst = ((*d as u64)<<24) | ((*c as u64)<<16) | ((*b as u64)<<8) | *a as u64;
                                let d = decode_inst(rv_isa::rv32, 0, inst);
                                if d.op != rv_op::illegal {
                                    add(&mut stats, d.op, inst);
                                    if verbose {
                                        println!("{}", format_inst(32, &d));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { panic!("Unhandled bit size {}", sz); }
    }
    if !verbose { // if not verbose, provide summary
        for (op, bits) in &stats {
            println!("{:10} {}", op.name(), bits.to_string(sz));
        }
    }
}
