use types::*;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct rv_comp_data {
    pub op: rv_op,
    pub constraints: &'static [rvc_constraint],
}

pub struct rvcd { }

impl rvcd {
    pub const none: u8 = 0x0;
    pub const imm_nz: u8 = 0x1;
    pub const imm_nz_hint: u8 = 0x2;
}

pub struct rv_opcode_data {
    pub name: &'static str,
    pub codec: rv_codec,
    pub format: &'static str,
    pub pseudo: &'static [rv_comp_data],
    pub decomp_rv32: rv_op,
    pub decomp_rv64: rv_op,
    pub decomp_rv128: rv_op,
    pub decomp_data: u8,
}

/* instruction formats */

const rv_fmt_none: &str                 = "O\t";
const rv_fmt_rs1: &str                  = "O\t1";
const rv_fmt_offset: &str               = "O\to";
const rv_fmt_pred_succ: &str            = "O\tp,s";
const rv_fmt_rs1_rs2: &str              = "O\t1,2";
const rv_fmt_rd_imm: &str               = "O\t0,i";
const rv_fmt_rd_offset: &str            = "O\t0,o";
const rv_fmt_rd_rs1_rs2: &str           = "O\t0,1,2";
const rv_fmt_frd_rs1: &str              = "O\t3,1";
const rv_fmt_rd_frs1: &str              = "O\t0,4";
const rv_fmt_rd_frs1_frs2: &str         = "O\t0,4,5";
const rv_fmt_frd_frs1_frs2: &str        = "O\t3,4,5";
const rv_fmt_rm_frd_frs1: &str          = "O\tr,3,4";
const rv_fmt_rm_frd_rs1: &str           = "O\tr,3,1";
const rv_fmt_rm_rd_frs1: &str           = "O\tr,0,4";
const rv_fmt_rm_frd_frs1_frs2: &str     = "O\tr,3,4,5";
const rv_fmt_rm_frd_frs1_frs2_frs3: &str = "O\tr,3,4,5,6";
const rv_fmt_rd_rs1_imm: &str           = "O\t0,1,i";
const rv_fmt_rd_rs1_offset: &str        = "O\t0,1,i";
const rv_fmt_rd_offset_rs1: &str        = "O\t0,i(1)";
const rv_fmt_frd_offset_rs1: &str       = "O\t3,i(1)";
const rv_fmt_rd_csr_rs1: &str           = "O\t0,c,1";
const rv_fmt_rd_csr_zimm: &str          = "O\t0,c,7";
const rv_fmt_rs2_offset_rs1: &str       = "O\t2,i(1)";
const rv_fmt_frs2_offset_rs1: &str      = "O\t5,i(1)";
const rv_fmt_rs1_rs2_offset: &str       = "O\t1,2,o";
const rv_fmt_rs2_rs1_offset: &str       = "O\t2,1,o";
const rv_fmt_aqrl_rd_rs2_rs1: &str      = "OAR\t0,2,(1)";
const rv_fmt_aqrl_rd_rs1: &str          = "OAR\t0,(1)";
const rv_fmt_rd: &str                   = "O\t0";
const rv_fmt_rd_zimm: &str              = "O\t0,7";
const rv_fmt_rd_rs1: &str               = "O\t0,1";
const rv_fmt_rd_rs2: &str               = "O\t0,2";
const rv_fmt_rs1_offset: &str           = "O\t1,o";
const rv_fmt_rs2_offset: &str           = "O\t2,o";

/* pseudo-instruction constraints */

use types::rvc_constraint as rvcc;

const rvcc_jal: &[rvcc] = &[ rvcc::rd_eq_ra ];
const rvcc_jalr: &[rvcc] = &[ rvcc::rd_eq_ra, rvcc::imm_eq_zero ];
const rvcc_nop: &[rvcc] = &[ rvcc::rd_eq_x0, rvcc::rs1_eq_x0, rvcc::imm_eq_zero ];
const rvcc_mv: &[rvcc] = &[ rvcc::imm_eq_zero ];
const rvcc_not: &[rvcc] = &[ rvcc::imm_eq_n1 ];
const rvcc_neg: &[rvcc] = &[ rvcc::rs1_eq_x0 ];
const rvcc_negw: &[rvcc] = &[ rvcc::rs1_eq_x0 ];
const rvcc_sext_w: &[rvcc] = &[ rvcc::imm_eq_zero ];
const rvcc_seqz: &[rvcc] = &[ rvcc::imm_eq_p1 ];
const rvcc_snez: &[rvcc] = &[ rvcc::rs1_eq_x0 ];
const rvcc_sltz: &[rvcc] = &[ rvcc::rs2_eq_x0 ];
const rvcc_sgtz: &[rvcc] = &[ rvcc::rs1_eq_x0 ];
const rvcc_fmv_s: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_fabs_s: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_fneg_s: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_fmv_d: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_fabs_d: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_fneg_d: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_fmv_q: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_fabs_q: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_fneg_q: &[rvcc] = &[ rvcc::rs2_eq_rs1 ];
const rvcc_beqz: &[rvcc] = &[ rvcc::rs2_eq_x0 ];
const rvcc_bnez: &[rvcc] = &[ rvcc::rs2_eq_x0 ];
const rvcc_blez: &[rvcc] = &[ rvcc::rs1_eq_x0 ];
const rvcc_bgez: &[rvcc] = &[ rvcc::rs2_eq_x0 ];
const rvcc_bltz: &[rvcc] = &[ rvcc::rs2_eq_x0 ];
const rvcc_bgtz: &[rvcc] = &[ rvcc::rs1_eq_x0 ];
const rvcc_ble: &[rvcc] = &[ ];
const rvcc_bleu: &[rvcc] = &[ ];
const rvcc_bgt: &[rvcc] = &[ ];
const rvcc_bgtu: &[rvcc] = &[ ];
const rvcc_j: &[rvcc] = &[ rvcc::rd_eq_x0 ];
const rvcc_ret: &[rvcc] = &[ rvcc::rd_eq_x0, rvcc::rs1_eq_ra ];
const rvcc_jr: &[rvcc] = &[ rvcc::rd_eq_x0, rvcc::imm_eq_zero ];
const rvcc_rdcycle: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc00 ];
const rvcc_rdtime: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc01 ];
const rvcc_rdinstret: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc02 ];
const rvcc_rdcycleh: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc80 ];
const rvcc_rdtimeh: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc81 ];
const rvcc_rdinstreth: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc80 ];
const rvcc_frcsr: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0x003 ];
const rvcc_frrm: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0x002 ];
const rvcc_frflags: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0x001 ];
const rvcc_fscsr: &[rvcc] = &[ rvcc::csr_eq_0x003 ];
const rvcc_fsrm: &[rvcc] = &[ rvcc::csr_eq_0x002 ];
const rvcc_fsflags: &[rvcc] = &[ rvcc::csr_eq_0x001 ];
const rvcc_fsrmi: &[rvcc] = &[ rvcc::csr_eq_0x002 ];
const rvcc_fsflagsi: &[rvcc] = &[ rvcc::csr_eq_0x001 ];

/* pseudo-instruction metadata */

const rvcp_none: &[rv_comp_data] = &[
];

const rvcp_jal: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::j, constraints: rvcc_j },
    rv_comp_data { op: rv_op::jal, constraints: rvcc_jal },
];

const rvcp_jalr: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::ret, constraints: rvcc_ret },
    rv_comp_data { op: rv_op::jr, constraints: rvcc_jr },
    rv_comp_data { op: rv_op::jalr, constraints: rvcc_jalr },
];

const rvcp_beq: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::beqz, constraints: rvcc_beqz },
];

const rvcp_bne: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::bnez, constraints: rvcc_bnez },
];

const rvcp_blt: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::bltz, constraints: rvcc_bltz },
    rv_comp_data { op: rv_op::bgtz, constraints: rvcc_bgtz },
    rv_comp_data { op: rv_op::bgt, constraints: rvcc_bgt },
];

const rvcp_bge: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::blez, constraints: rvcc_blez },
    rv_comp_data { op: rv_op::bgez, constraints: rvcc_bgez },
    rv_comp_data { op: rv_op::ble, constraints: rvcc_ble },
];

const rvcp_bltu: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::bgtu, constraints: rvcc_bgtu },
];

const rvcp_bgeu: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::bleu, constraints: rvcc_bleu },
];

const rvcp_addi: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::nop, constraints: rvcc_nop },
    rv_comp_data { op: rv_op::mv, constraints: rvcc_mv },
];

const rvcp_sltiu: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::seqz, constraints: rvcc_seqz },
];

const rvcp_xori: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::not, constraints: rvcc_not },
];

const rvcp_sub: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::neg, constraints: rvcc_neg },
];

const rvcp_slt: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::sltz, constraints: rvcc_sltz },
    rv_comp_data { op: rv_op::sgtz, constraints: rvcc_sgtz },
];

const rvcp_sltu: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::snez, constraints: rvcc_snez },
];

const rvcp_addiw: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::sext_w, constraints: rvcc_sext_w },
];

const rvcp_subw: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::negw, constraints: rvcc_negw },
];

const rvcp_csrrw: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fscsr, constraints: rvcc_fscsr },
    rv_comp_data { op: rv_op::fsrm, constraints: rvcc_fsrm },
    rv_comp_data { op: rv_op::fsflags, constraints: rvcc_fsflags },
];

const rvcp_csrrs: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::rdcycle, constraints: rvcc_rdcycle },
    rv_comp_data { op: rv_op::rdtime, constraints: rvcc_rdtime },
    rv_comp_data { op: rv_op::rdinstret, constraints: rvcc_rdinstret },
    rv_comp_data { op: rv_op::rdcycleh, constraints: rvcc_rdcycleh },
    rv_comp_data { op: rv_op::rdtimeh, constraints: rvcc_rdtimeh },
    rv_comp_data { op: rv_op::rdinstreth, constraints: rvcc_rdinstreth },
    rv_comp_data { op: rv_op::frcsr, constraints: rvcc_frcsr },
    rv_comp_data { op: rv_op::frrm, constraints: rvcc_frrm },
    rv_comp_data { op: rv_op::frflags, constraints: rvcc_frflags },
];

const rvcp_csrrwi: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fsrmi, constraints: rvcc_fsrmi },
    rv_comp_data { op: rv_op::fsflagsi, constraints: rvcc_fsflagsi },
];

const rvcp_fsgnj_s: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fmv_s, constraints: rvcc_fmv_s },
];

const rvcp_fsgnjn_s: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fneg_s, constraints: rvcc_fneg_s },
];

const rvcp_fsgnjx_s: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fabs_s, constraints: rvcc_fabs_s },
];

const rvcp_fsgnj_d: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fmv_d, constraints: rvcc_fmv_d },
];

const rvcp_fsgnjn_d: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fneg_d, constraints: rvcc_fneg_d },
];

const rvcp_fsgnjx_d: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fabs_d, constraints: rvcc_fabs_d },
];

const rvcp_fsgnj_q: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fmv_q, constraints: rvcc_fmv_q },
];

const rvcp_fsgnjn_q: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fneg_q, constraints: rvcc_fneg_q },
];

const rvcp_fsgnjx_q: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fabs_q, constraints: rvcc_fabs_q },
];

/* instruction metadata */

const noop: rv_op = rv_op::illegal;

// Must be in the same order as enum rv_op 
pub const opcode_data: &[rv_opcode_data] = &[
    rv_opcode_data { name: "illegal", codec: rv_codec::illegal, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lui", codec: rv_codec::u, format: rv_fmt_rd_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "auipc", codec: rv_codec::u, format: rv_fmt_rd_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "jal", codec: rv_codec::uj, format: rv_fmt_rd_offset, pseudo: rvcp_jal, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "jalr", codec: rv_codec::i, format: rv_fmt_rd_rs1_offset, pseudo: rvcp_jalr, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "beq", codec: rv_codec::sb, format: rv_fmt_rs1_rs2_offset, pseudo: rvcp_beq, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bne", codec: rv_codec::sb, format: rv_fmt_rs1_rs2_offset, pseudo: rvcp_bne, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "blt", codec: rv_codec::sb, format: rv_fmt_rs1_rs2_offset, pseudo: rvcp_blt, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bge", codec: rv_codec::sb, format: rv_fmt_rs1_rs2_offset, pseudo: rvcp_bge, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bltu", codec: rv_codec::sb, format: rv_fmt_rs1_rs2_offset, pseudo: rvcp_bltu, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bgeu", codec: rv_codec::sb, format: rv_fmt_rs1_rs2_offset, pseudo: rvcp_bgeu, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lb", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lh", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lw", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lbu", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lhu", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sb", codec: rv_codec::s, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sh", codec: rv_codec::s, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sw", codec: rv_codec::s, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "addi", codec: rv_codec::i, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_addi, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "slti", codec: rv_codec::i, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sltiu", codec: rv_codec::i, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_sltiu, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "xori", codec: rv_codec::i, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_xori, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "ori", codec: rv_codec::i, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "andi", codec: rv_codec::i, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "slli", codec: rv_codec::i_sh7, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "srli", codec: rv_codec::i_sh7, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "srai", codec: rv_codec::i_sh7, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "add", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sub", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_sub, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sll", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "slt", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_slt, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sltu", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_sltu, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "xor", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "srl", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sra", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "or", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "and", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fence", codec: rv_codec::r_f, format: rv_fmt_pred_succ, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fence.i", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lwu", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "ld", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sd", codec: rv_codec::s, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "addiw", codec: rv_codec::i, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_addiw, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "slliw", codec: rv_codec::i_sh5, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "srliw", codec: rv_codec::i_sh5, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sraiw", codec: rv_codec::i_sh5, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "addw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "subw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_subw, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sllw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "srlw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sraw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "ldu", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lq", codec: rv_codec::i, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sq", codec: rv_codec::s, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "addid", codec: rv_codec::i, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sllid", codec: rv_codec::i_sh6, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "srlid", codec: rv_codec::i_sh6, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sraid", codec: rv_codec::i_sh6, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "addd", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "subd", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "slld", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "srld", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "srad", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "mul", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "mulh", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "mulhsu", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "mulhu", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "div", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "divu", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "rem", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "remu", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "mulw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "divw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "divuw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "remw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "remuw", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "muld", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "divd", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "divud", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "remd", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "remud", codec: rv_codec::r, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lr.w", codec: rv_codec::r_l, format: rv_fmt_aqrl_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sc.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoswap.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoadd.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoxor.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoor.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoand.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomin.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomax.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amominu.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomaxu.w", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lr.d", codec: rv_codec::r_l, format: rv_fmt_aqrl_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sc.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoswap.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoadd.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoxor.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoor.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoand.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomin.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomax.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amominu.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomaxu.d", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "lr.q", codec: rv_codec::r_l, format: rv_fmt_aqrl_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sc.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoswap.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoadd.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoxor.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoor.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amoand.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomin.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomax.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amominu.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "amomaxu.q", codec: rv_codec::r_a, format: rv_fmt_aqrl_rd_rs2_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "ecall", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "ebreak", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "uret", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sret", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "hret", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "mret", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "dret", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sfence.vm", codec: rv_codec::r, format: rv_fmt_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sfence.vma", codec: rv_codec::r, format: rv_fmt_rs1_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "wfi", codec: rv_codec::none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "csrrw", codec: rv_codec::i_csr, format: rv_fmt_rd_csr_rs1, pseudo: rvcp_csrrw, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "csrrs", codec: rv_codec::i_csr, format: rv_fmt_rd_csr_rs1, pseudo: rvcp_csrrs, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "csrrc", codec: rv_codec::i_csr, format: rv_fmt_rd_csr_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "csrrwi", codec: rv_codec::i_csr, format: rv_fmt_rd_csr_zimm, pseudo: rvcp_csrrwi, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "csrrsi", codec: rv_codec::i_csr, format: rv_fmt_rd_csr_zimm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "csrrci", codec: rv_codec::i_csr, format: rv_fmt_rd_csr_zimm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "flw", codec: rv_codec::i, format: rv_fmt_frd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsw", codec: rv_codec::s, format: rv_fmt_frs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmadd.s", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmsub.s", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fnmsub.s", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fnmadd.s", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fadd.s", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsub.s", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmul.s", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fdiv.s", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnj.s", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnj_s, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnjn.s", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnjn_s, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnjx.s", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnjx_s, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmin.s", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmax.s", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsqrt.s", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fle.s", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "flt.s", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "feq.s", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.w.s", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.wu.s", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.s.w", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.s.wu", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.x.s", codec: rv_codec::r, format: rv_fmt_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fclass.s", codec: rv_codec::r, format: rv_fmt_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.s.x", codec: rv_codec::r, format: rv_fmt_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.l.s", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.lu.s", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.s.l", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.s.lu", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fld", codec: rv_codec::i, format: rv_fmt_frd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsd", codec: rv_codec::s, format: rv_fmt_frs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmadd.d", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmsub.d", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fnmsub.d", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fnmadd.d", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fadd.d", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsub.d", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmul.d", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fdiv.d", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnj.d", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnj_d, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnjn.d", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnjn_d, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnjx.d", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnjx_d, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmin.d", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmax.d", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.s.d", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.d.s", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsqrt.d", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fle.d", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "flt.d", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "feq.d", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.w.d", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.wu.d", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.d.w", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.d.wu", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fclass.d", codec: rv_codec::r, format: rv_fmt_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.l.d", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.lu.d", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.x.d", codec: rv_codec::r, format: rv_fmt_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.d.l", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.d.lu", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.d.x", codec: rv_codec::r, format: rv_fmt_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "flq", codec: rv_codec::i, format: rv_fmt_frd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsq", codec: rv_codec::s, format: rv_fmt_frs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmadd.q", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmsub.q", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fnmsub.q", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fnmadd.q", codec: rv_codec::r4_m, format: rv_fmt_rm_frd_frs1_frs2_frs3, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fadd.q", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsub.q", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmul.q", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fdiv.q", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnj.q", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnj_q, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnjn.q", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnjn_q, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsgnjx.q", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_fsgnjx_q, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmin.q", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmax.q", codec: rv_codec::r, format: rv_fmt_frd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.s.q", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.q.s", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.d.q", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.q.d", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsqrt.q", codec: rv_codec::r_m, format: rv_fmt_rm_frd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fle.q", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "flt.q", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "feq.q", codec: rv_codec::r, format: rv_fmt_rd_frs1_frs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.w.q", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.wu.q", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.q.w", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.q.wu", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fclass.q", codec: rv_codec::r, format: rv_fmt_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.l.q", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.lu.q", codec: rv_codec::r_m, format: rv_fmt_rm_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.q.l", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fcvt.q.lu", codec: rv_codec::r_m, format: rv_fmt_rm_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.x.q", codec: rv_codec::r, format: rv_fmt_rd_frs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.q.x", codec: rv_codec::r, format: rv_fmt_frd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.addi4spn", codec: rv_codec::ciw_4spn, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: rv_op::addi, decomp_rv64: rv_op::addi, decomp_rv128: rv_op::addi, decomp_data: rvcd::imm_nz },
    rv_opcode_data { name: "c.fld", codec: rv_codec::cl_ld, format: rv_fmt_frd_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::fld, decomp_rv64: rv_op::fld, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.lw", codec: rv_codec::cl_lw, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::lw, decomp_rv64: rv_op::lw, decomp_rv128: rv_op::lw, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.flw", codec: rv_codec::cl_lw, format: rv_fmt_frd_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::flw, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.fsd", codec: rv_codec::cs_sd, format: rv_fmt_frs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::fsd, decomp_rv64: rv_op::fsd, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.sw", codec: rv_codec::cs_sw, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::sw, decomp_rv64: rv_op::sw, decomp_rv128: rv_op::sw, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.fsw", codec: rv_codec::cs_sw, format: rv_fmt_frs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::fsw, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.nop", codec: rv_codec::ci_none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: rv_op::addi, decomp_rv64: rv_op::addi, decomp_rv128: rv_op::addi, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.addi", codec: rv_codec::ci, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: rv_op::addi, decomp_rv64: rv_op::addi, decomp_rv128: rv_op::addi, decomp_data: rvcd::imm_nz_hint },
    rv_opcode_data { name: "c.jal", codec: rv_codec::cj_jal, format: rv_fmt_rd_offset, pseudo: rvcp_none, decomp_rv32: rv_op::jal, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.li", codec: rv_codec::ci_li, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: rv_op::addi, decomp_rv64: rv_op::addi, decomp_rv128: rv_op::addi, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.addi16sp", codec: rv_codec::ci_16sp, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: rv_op::addi, decomp_rv64: rv_op::addi, decomp_rv128: rv_op::addi, decomp_data: rvcd::imm_nz },
    rv_opcode_data { name: "c.lui", codec: rv_codec::ci_lui, format: rv_fmt_rd_imm, pseudo: rvcp_none, decomp_rv32: rv_op::lui, decomp_rv64: rv_op::lui, decomp_rv128: rv_op::lui, decomp_data: rvcd::imm_nz },
    rv_opcode_data { name: "c.srli", codec: rv_codec::cb_sh6, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: rv_op::srli, decomp_rv64: rv_op::srli, decomp_rv128: rv_op::srli, decomp_data: rvcd::imm_nz },
    rv_opcode_data { name: "c.srai", codec: rv_codec::cb_sh6, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: rv_op::srai, decomp_rv64: rv_op::srai, decomp_rv128: rv_op::srai, decomp_data: rvcd::imm_nz },
    rv_opcode_data { name: "c.andi", codec: rv_codec::cb_imm, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: rv_op::andi, decomp_rv64: rv_op::andi, decomp_rv128: rv_op::andi, decomp_data: rvcd::imm_nz },
    rv_opcode_data { name: "c.sub", codec: rv_codec::cs, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: rv_op::sub, decomp_rv64: rv_op::sub, decomp_rv128: rv_op::sub, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.xor", codec: rv_codec::cs, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: rv_op::xor, decomp_rv64: rv_op::xor, decomp_rv128: rv_op::xor, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.or", codec: rv_codec::cs, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: rv_op::or, decomp_rv64: rv_op::or, decomp_rv128: rv_op::or, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.and", codec: rv_codec::cs, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: rv_op::and, decomp_rv64: rv_op::and, decomp_rv128: rv_op::and, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.subw", codec: rv_codec::cs, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: rv_op::subw, decomp_rv64: rv_op::subw, decomp_rv128: rv_op::subw, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.addw", codec: rv_codec::cs, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: rv_op::addw, decomp_rv64: rv_op::addw, decomp_rv128: rv_op::addw, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.j", codec: rv_codec::cj, format: rv_fmt_rd_offset, pseudo: rvcp_none, decomp_rv32: rv_op::jal, decomp_rv64: rv_op::jal, decomp_rv128: rv_op::jal, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.beqz", codec: rv_codec::cb, format: rv_fmt_rs1_rs2_offset, pseudo: rvcp_none, decomp_rv32: rv_op::beq, decomp_rv64: rv_op::beq, decomp_rv128: rv_op::beq, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.bnez", codec: rv_codec::cb, format: rv_fmt_rs1_rs2_offset, pseudo: rvcp_none, decomp_rv32: rv_op::bne, decomp_rv64: rv_op::bne, decomp_rv128: rv_op::bne, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.slli", codec: rv_codec::ci_sh6, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: rv_op::slli, decomp_rv64: rv_op::slli, decomp_rv128: rv_op::slli, decomp_data: rvcd::imm_nz },
    rv_opcode_data { name: "c.fldsp", codec: rv_codec::ci_ldsp, format: rv_fmt_frd_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::fld, decomp_rv64: rv_op::fld, decomp_rv128: rv_op::fld, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.lwsp", codec: rv_codec::ci_lwsp, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::lw, decomp_rv64: rv_op::lw, decomp_rv128: rv_op::lw, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.flwsp", codec: rv_codec::ci_lwsp, format: rv_fmt_frd_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::flw, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.jr", codec: rv_codec::cr_jr, format: rv_fmt_rd_rs1_offset, pseudo: rvcp_none, decomp_rv32: rv_op::jalr, decomp_rv64: rv_op::jalr, decomp_rv128: rv_op::jalr, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.mv", codec: rv_codec::cr_mv, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: rv_op::addi, decomp_rv64: rv_op::addi, decomp_rv128: rv_op::addi, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.ebreak", codec: rv_codec::ci_none, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: rv_op::ebreak, decomp_rv64: rv_op::ebreak, decomp_rv128: rv_op::ebreak, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.jalr", codec: rv_codec::cr_jalr, format: rv_fmt_rd_rs1_offset, pseudo: rvcp_none, decomp_rv32: rv_op::jalr, decomp_rv64: rv_op::jalr, decomp_rv128: rv_op::jalr, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.add", codec: rv_codec::cr, format: rv_fmt_rd_rs1_rs2, pseudo: rvcp_none, decomp_rv32: rv_op::add, decomp_rv64: rv_op::add, decomp_rv128: rv_op::add, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.fsdsp", codec: rv_codec::css_sdsp, format: rv_fmt_frs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::fsd, decomp_rv64: rv_op::fsd, decomp_rv128: rv_op::fsd, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.swsp", codec: rv_codec::css_swsp, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::sw, decomp_rv64: rv_op::sw, decomp_rv128: rv_op::sw, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.fswsp", codec: rv_codec::css_swsp, format: rv_fmt_frs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: rv_op::fsw, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.ld", codec: rv_codec::cl_ld, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: rv_op::ld, decomp_rv128: rv_op::ld, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.sd", codec: rv_codec::cs_sd, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: rv_op::sd, decomp_rv128: rv_op::sd, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.addiw", codec: rv_codec::ci, format: rv_fmt_rd_rs1_imm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: rv_op::addiw, decomp_rv128: rv_op::addiw, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.ldsp", codec: rv_codec::ci_ldsp, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: rv_op::ld, decomp_rv128: rv_op::ld, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.sdsp", codec: rv_codec::css_sdsp, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: rv_op::sd, decomp_rv128: rv_op::sd, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.lq", codec: rv_codec::cl_lq, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: rv_op::lq, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.sq", codec: rv_codec::cs_sq, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: rv_op::sq, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.lqsp", codec: rv_codec::ci_lqsp, format: rv_fmt_rd_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: rv_op::lq, decomp_data: rvcd::none },
    rv_opcode_data { name: "c.sqsp", codec: rv_codec::css_sqsp, format: rv_fmt_rs2_offset_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: rv_op::sq, decomp_data: rvcd::none },
    rv_opcode_data { name: "nop", codec: rv_codec::i, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "mv", codec: rv_codec::i, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "not", codec: rv_codec::i, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "neg", codec: rv_codec::r, format: rv_fmt_rd_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "negw", codec: rv_codec::r, format: rv_fmt_rd_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sext.w", codec: rv_codec::i, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "seqz", codec: rv_codec::i, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "snez", codec: rv_codec::r, format: rv_fmt_rd_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sltz", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "sgtz", codec: rv_codec::r, format: rv_fmt_rd_rs2, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.s", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fabs.s", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fneg.s", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.d", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fabs.d", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fneg.d", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fmv.q", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fabs.q", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fneg.q", codec: rv_codec::r, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "beqz", codec: rv_codec::sb, format: rv_fmt_rs1_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bnez", codec: rv_codec::sb, format: rv_fmt_rs1_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "blez", codec: rv_codec::sb, format: rv_fmt_rs2_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bgez", codec: rv_codec::sb, format: rv_fmt_rs1_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bltz", codec: rv_codec::sb, format: rv_fmt_rs1_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bgtz", codec: rv_codec::sb, format: rv_fmt_rs2_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "ble", codec: rv_codec::sb, format: rv_fmt_rs2_rs1_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bleu", codec: rv_codec::sb, format: rv_fmt_rs2_rs1_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bgt", codec: rv_codec::sb, format: rv_fmt_rs2_rs1_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "bgtu", codec: rv_codec::sb, format: rv_fmt_rs2_rs1_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "j", codec: rv_codec::uj, format: rv_fmt_offset, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "ret", codec: rv_codec::i, format: rv_fmt_none, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "jr", codec: rv_codec::i, format: rv_fmt_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "rdcycle", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "rdtime", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "rdinstret", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "rdcycleh", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "rdtimeh", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "rdinstreth", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "frcsr", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "frrm", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "frflags", codec: rv_codec::i_csr, format: rv_fmt_rd, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fscsr", codec: rv_codec::i_csr, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsrm", codec: rv_codec::i_csr, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsflags", codec: rv_codec::i_csr, format: rv_fmt_rd_rs1, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsrmi", codec: rv_codec::i_csr, format: rv_fmt_rd_zimm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
    rv_opcode_data { name: "fsflagsi", codec: rv_codec::i_csr, format: rv_fmt_rd_zimm, pseudo: rvcp_none, decomp_rv32: noop, decomp_rv64: noop, decomp_rv128: noop, decomp_data: rvcd::none },
];
