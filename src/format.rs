use disasm::{decode_inst, inst_length};
use opcode_data::opcode_data;
use types::*;

/* register names */

const rv_ireg_name_sym: &[&str] = &[
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

const rv_freg_name_sym: &[&str] = &[
    "ft0", "ft1", "ft2", "ft3", "ft4", "ft5", "ft6", "ft7", "fs0", "fs1", "fa0", "fa1", "fa2",
    "fa3", "fa4", "fa5", "fa6", "fa7", "fs2", "fs3", "fs4", "fs5", "fs6", "fs7", "fs8", "fs9",
    "fs10", "fs11", "ft8", "ft9", "ft10", "ft11",
];

/* CSR names */

fn csr_name(csrno: i32) -> Option<&'static str> {
    match csrno {
        0x0000 => Some("ustatus"),
        0x0001 => Some("fflags"),
        0x0002 => Some("frm"),
        0x0003 => Some("fcsr"),
        0x0004 => Some("uie"),
        0x0005 => Some("utvec"),
        0x0040 => Some("uscratch"),
        0x0041 => Some("uepc"),
        0x0042 => Some("ucause"),
        0x0043 => Some("utval"),
        0x0044 => Some("uip"),
        0x0100 => Some("sstatus"),
        0x0102 => Some("sedeleg"),
        0x0103 => Some("sideleg"),
        0x0104 => Some("sie"),
        0x0105 => Some("stvec"),
        0x0106 => Some("scounteren"),
        0x0140 => Some("sscratch"),
        0x0141 => Some("sepc"),
        0x0142 => Some("scause"),
        0x0143 => Some("stval"),
        0x0144 => Some("sip"),
        0x0180 => Some("satp"),
        0x0200 => Some("hstatus"),
        0x0202 => Some("hedeleg"),
        0x0203 => Some("hideleg"),
        0x0204 => Some("hie"),
        0x0205 => Some("htvec"),
        0x0240 => Some("hscratch"),
        0x0241 => Some("hepc"),
        0x0242 => Some("hcause"),
        0x0243 => Some("hbadaddr"),
        0x0244 => Some("hip"),
        0x0300 => Some("mstatus"),
        0x0301 => Some("misa"),
        0x0302 => Some("medeleg"),
        0x0303 => Some("mideleg"),
        0x0304 => Some("mie"),
        0x0305 => Some("mtvec"),
        0x0306 => Some("mcounteren"),
        0x0320 => Some("mucounteren"),
        0x0321 => Some("mscounteren"),
        0x0322 => Some("mhcounteren"),
        0x0323 => Some("mhpmevent3"),
        0x0324 => Some("mhpmevent4"),
        0x0325 => Some("mhpmevent5"),
        0x0326 => Some("mhpmevent6"),
        0x0327 => Some("mhpmevent7"),
        0x0328 => Some("mhpmevent8"),
        0x0329 => Some("mhpmevent9"),
        0x032a => Some("mhpmevent10"),
        0x032b => Some("mhpmevent11"),
        0x032c => Some("mhpmevent12"),
        0x032d => Some("mhpmevent13"),
        0x032e => Some("mhpmevent14"),
        0x032f => Some("mhpmevent15"),
        0x0330 => Some("mhpmevent16"),
        0x0331 => Some("mhpmevent17"),
        0x0332 => Some("mhpmevent18"),
        0x0333 => Some("mhpmevent19"),
        0x0334 => Some("mhpmevent20"),
        0x0335 => Some("mhpmevent21"),
        0x0336 => Some("mhpmevent22"),
        0x0337 => Some("mhpmevent23"),
        0x0338 => Some("mhpmevent24"),
        0x0339 => Some("mhpmevent25"),
        0x033a => Some("mhpmevent26"),
        0x033b => Some("mhpmevent27"),
        0x033c => Some("mhpmevent28"),
        0x033d => Some("mhpmevent29"),
        0x033e => Some("mhpmevent30"),
        0x033f => Some("mhpmevent31"),
        0x0340 => Some("mscratch"),
        0x0341 => Some("mepc"),
        0x0342 => Some("mcause"),
        0x0343 => Some("mtval"),
        0x0344 => Some("mip"),
        0x0380 => Some("mbase"),
        0x0381 => Some("mbound"),
        0x0382 => Some("mibase"),
        0x0383 => Some("mibound"),
        0x0384 => Some("mdbase"),
        0x0385 => Some("mdbound"),
        0x03a0 => Some("pmpcfg0"),
        0x03a1 => Some("pmpcfg1"),
        0x03a2 => Some("pmpcfg2"),
        0x03a3 => Some("pmpcfg3"),
        0x03b0 => Some("pmpaddr0"),
        0x03b1 => Some("pmpaddr1"),
        0x03b2 => Some("pmpaddr2"),
        0x03b3 => Some("pmpaddr3"),
        0x03b4 => Some("pmpaddr4"),
        0x03b5 => Some("pmpaddr5"),
        0x03b6 => Some("pmpaddr6"),
        0x03b7 => Some("pmpaddr7"),
        0x03b8 => Some("pmpaddr8"),
        0x03b9 => Some("pmpaddr9"),
        0x03ba => Some("pmpaddr10"),
        0x03bb => Some("pmpaddr11"),
        0x03bc => Some("pmpaddr12"),
        0x03bd => Some("pmpaddr14"),
        0x03be => Some("pmpaddr13"),
        0x03bf => Some("pmpaddr15"),
        0x0780 => Some("mtohost"),
        0x0781 => Some("mfromhost"),
        0x0782 => Some("mreset"),
        0x0783 => Some("mipi"),
        0x0784 => Some("miobase"),
        0x07a0 => Some("tselect"),
        0x07a1 => Some("tdata1"),
        0x07a2 => Some("tdata2"),
        0x07a3 => Some("tdata3"),
        0x07b0 => Some("dcsr"),
        0x07b1 => Some("dpc"),
        0x07b2 => Some("dscratch"),
        0x0b00 => Some("mcycle"),
        0x0b01 => Some("mtime"),
        0x0b02 => Some("minstret"),
        0x0b03 => Some("mhpmcounter3"),
        0x0b04 => Some("mhpmcounter4"),
        0x0b05 => Some("mhpmcounter5"),
        0x0b06 => Some("mhpmcounter6"),
        0x0b07 => Some("mhpmcounter7"),
        0x0b08 => Some("mhpmcounter8"),
        0x0b09 => Some("mhpmcounter9"),
        0x0b0a => Some("mhpmcounter10"),
        0x0b0b => Some("mhpmcounter11"),
        0x0b0c => Some("mhpmcounter12"),
        0x0b0d => Some("mhpmcounter13"),
        0x0b0e => Some("mhpmcounter14"),
        0x0b0f => Some("mhpmcounter15"),
        0x0b10 => Some("mhpmcounter16"),
        0x0b11 => Some("mhpmcounter17"),
        0x0b12 => Some("mhpmcounter18"),
        0x0b13 => Some("mhpmcounter19"),
        0x0b14 => Some("mhpmcounter20"),
        0x0b15 => Some("mhpmcounter21"),
        0x0b16 => Some("mhpmcounter22"),
        0x0b17 => Some("mhpmcounter23"),
        0x0b18 => Some("mhpmcounter24"),
        0x0b19 => Some("mhpmcounter25"),
        0x0b1a => Some("mhpmcounter26"),
        0x0b1b => Some("mhpmcounter27"),
        0x0b1c => Some("mhpmcounter28"),
        0x0b1d => Some("mhpmcounter29"),
        0x0b1e => Some("mhpmcounter30"),
        0x0b1f => Some("mhpmcounter31"),
        0x0b80 => Some("mcycleh"),
        0x0b81 => Some("mtimeh"),
        0x0b82 => Some("minstreth"),
        0x0b83 => Some("mhpmcounter3h"),
        0x0b84 => Some("mhpmcounter4h"),
        0x0b85 => Some("mhpmcounter5h"),
        0x0b86 => Some("mhpmcounter6h"),
        0x0b87 => Some("mhpmcounter7h"),
        0x0b88 => Some("mhpmcounter8h"),
        0x0b89 => Some("mhpmcounter9h"),
        0x0b8a => Some("mhpmcounter10h"),
        0x0b8b => Some("mhpmcounter11h"),
        0x0b8c => Some("mhpmcounter12h"),
        0x0b8d => Some("mhpmcounter13h"),
        0x0b8e => Some("mhpmcounter14h"),
        0x0b8f => Some("mhpmcounter15h"),
        0x0b90 => Some("mhpmcounter16h"),
        0x0b91 => Some("mhpmcounter17h"),
        0x0b92 => Some("mhpmcounter18h"),
        0x0b93 => Some("mhpmcounter19h"),
        0x0b94 => Some("mhpmcounter20h"),
        0x0b95 => Some("mhpmcounter21h"),
        0x0b96 => Some("mhpmcounter22h"),
        0x0b97 => Some("mhpmcounter23h"),
        0x0b98 => Some("mhpmcounter24h"),
        0x0b99 => Some("mhpmcounter25h"),
        0x0b9a => Some("mhpmcounter26h"),
        0x0b9b => Some("mhpmcounter27h"),
        0x0b9c => Some("mhpmcounter28h"),
        0x0b9d => Some("mhpmcounter29h"),
        0x0b9e => Some("mhpmcounter30h"),
        0x0b9f => Some("mhpmcounter31h"),
        0x0c00 => Some("cycle"),
        0x0c01 => Some("time"),
        0x0c02 => Some("instret"),
        0x0c80 => Some("cycleh"),
        0x0c81 => Some("timeh"),
        0x0c82 => Some("instreth"),
        0x0d00 => Some("scycle"),
        0x0d01 => Some("stime"),
        0x0d02 => Some("sinstret"),
        0x0d80 => Some("scycleh"),
        0x0d81 => Some("stimeh"),
        0x0d82 => Some("sinstreth"),
        0x0e00 => Some("hcycle"),
        0x0e01 => Some("htime"),
        0x0e02 => Some("hinstret"),
        0x0e80 => Some("hcycleh"),
        0x0e81 => Some("htimeh"),
        0x0e82 => Some("hinstreth"),
        0x0f11 => Some("mvendorid"),
        0x0f12 => Some("marchid"),
        0x0f13 => Some("mimpid"),
        0x0f14 => Some("mhartid"),
        _ => None,
    }
}

/* format instruction */

pub fn format_inst(tab: usize, dec: &rv_decode) -> String {
    let mut buf: String;
    match inst_length(dec.inst) {
        2 => {
            buf = format!("{:04x}              ", dec.inst);
        }
        4 => {
            buf = format!("{:08x}          ", dec.inst);
        }
        6 => {
            buf = format!("{:012x}      ", dec.inst);
        }
        _ => {
            buf = format!("{:016x}  ", dec.inst);
        }
    };

    for ch in opcode_data[dec.op as usize].format.chars() {
        match ch {
            'O' => {
                buf.push_str(opcode_data[dec.op as usize].name);
            }
            '(' => {
                buf.push('(');
            }
            ',' => {
                buf.push(',');
            }
            ')' => {
                buf.push(')');
            }
            '0' => {
                buf.push_str(rv_ireg_name_sym[dec.rd as usize]);
            }
            '1' => {
                buf.push_str(rv_ireg_name_sym[dec.rs1 as usize]);
            }
            '2' => {
                buf.push_str(rv_ireg_name_sym[dec.rs2 as usize]);
            }
            '3' => {
                buf.push_str(rv_freg_name_sym[dec.rd as usize]);
            }
            '4' => {
                buf.push_str(rv_freg_name_sym[dec.rs1 as usize]);
            }
            '5' => {
                buf.push_str(rv_freg_name_sym[dec.rs2 as usize]);
            }
            '6' => {
                buf.push_str(rv_freg_name_sym[dec.rs3 as usize]);
            }
            '7' => {
                buf.push_str(&format!("{}", dec.rs1));
            }
            'i' => {
                buf.push_str(&format!("{}", dec.imm));
            }
            'o' => {
                buf.push_str(&format!("{}", dec.imm));
                while buf.len() < tab * 2 {
                    buf.push(' ');
                }
                buf.push_str(&format!("# 0x{:x}", dec.pc + (dec.imm as u64)));
            }
            'c' => {
                if let Some(name) = csr_name(dec.imm & 0xfff) {
                    buf.push_str(name);
                } else {
                    buf.push_str(&format!("0x{:03x}", dec.imm & 0xfff));
                }
            }
            'r' => buf.push_str(match dec.rm {
                rv_rm::rne => "rne",
                rv_rm::rtz => "rtz",
                rv_rm::rdn => "rdn",
                rv_rm::rup => "rup",
                rv_rm::rmm => "rmm",
                rv_rm::dyn => "dyn",
                _ => "inv",
            }),
            'p' => {
                if (dec.pred & rv_fence::i) != 0 {
                    buf.push('i');
                }
                if (dec.pred & rv_fence::o) != 0 {
                    buf.push('o');
                }
                if (dec.pred & rv_fence::r) != 0 {
                    buf.push('r');
                }
                if (dec.pred & rv_fence::w) != 0 {
                    buf.push('w');
                }
            }
            's' => {
                if (dec.succ & rv_fence::i) != 0 {
                    buf.push('i');
                }
                if (dec.succ & rv_fence::o) != 0 {
                    buf.push('o');
                }
                if (dec.succ & rv_fence::r) != 0 {
                    buf.push('r');
                }
                if (dec.succ & rv_fence::w) != 0 {
                    buf.push('w');
                }
            }
            '\t' => {
                while buf.len() < tab {
                    buf.push(' ');
                }
            }
            'A' => {
                if dec.aq {
                    buf.push_str(".aq");
                }
            }
            'R' => {
                if dec.rl {
                    buf.push_str(".rl");
                }
            }
            _ => {}
        }
    }

    buf
}

pub fn disasm_inst(isa: rv_isa, pc: u64, inst: rv_inst) -> String {
    format_inst(32, &decode_inst(isa, pc, inst))
}

#[cfg(test)]
mod tests {
    use super::*;

    const inst_arr: &[(u64, &'static str)] = &[
        (0x0, "0000              illegal       "),
        (0x1, "0001              nop           "),
        (0xd, "000d              addi          zero,zero,3"),
        (0x401, "0401              mv            s0,s0"),
        (0x404, "0404              addi          s1,sp,512"),
        (0x405, "0405              addi          s0,s0,1"),
        (
            0xf1402573,
            "f1402573          csrrs         a0,mhartid,zero",
        ),
        (
            0x597,
            "00000597          auipc         a1,0                            # 0x10088",
        ),
        (0x204002b7, "204002b7          lui           t0,541065216"),
        (0x13, "00000013          nop           "),
    ];

    #[test]
    fn basic_tests() {
        let mut pc: u64 = 0x10078;
        for (inst, expected) in inst_arr {
            let formatted = disasm_inst(rv_isa::rv64, pc, *inst);
            assert_eq!(&formatted, expected);
            pc = pc + inst_length(*inst) as u64;
        }
    }
}
