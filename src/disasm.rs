use opcode_data::opcode_data;
use types::*;

/* decode opcode */

fn decode_inst_opcode(inst: rv_inst, isa: rv_isa) -> rv_op {
    use types::rv_isa::*;
    match (inst >> 0) & 0b11 {
        0 => match (inst >> 13) & 0b111 {
            0 => rv_op::c_addi4spn,
            1 => {
                if isa == rv128 {
                    rv_op::c_lq
                } else {
                    rv_op::c_fld
                }
            }
            2 => rv_op::c_lw,
            3 => {
                if isa == rv32 {
                    rv_op::c_flw
                } else {
                    rv_op::c_ld
                }
            }
            5 => {
                if isa == rv128 {
                    rv_op::c_sq
                } else {
                    rv_op::c_fsd
                }
            }
            6 => rv_op::c_sw,
            7 => {
                if isa == rv32 {
                    rv_op::c_fsw
                } else {
                    rv_op::c_sd
                }
            }
            _ => rv_op::illegal,
        },
        1 => match (inst >> 13) & 0b111 {
            0 => match (inst >> 2) & 0b11111111111 {
                0 => rv_op::c_nop,
                _ => rv_op::c_addi,
            },
            1 => {
                if isa == rv32 {
                    rv_op::c_jal
                } else {
                    rv_op::c_addiw
                }
            }
            2 => rv_op::c_li,
            3 => match (inst >> 7) & 0b11111 {
                2 => rv_op::c_addi16sp,
                _ => rv_op::c_lui,
            },
            4 => match (inst >> 10) & 0b11 {
                0 => rv_op::c_srli,
                1 => rv_op::c_srai,
                2 => rv_op::c_andi,
                3 => match ((inst >> 10) & 0b100) | ((inst >> 5) & 0b011) {
                    0 => rv_op::c_sub,
                    1 => rv_op::c_xor,
                    2 => rv_op::c_or,
                    3 => rv_op::c_and,
                    4 => rv_op::c_subw,
                    5 => rv_op::c_addw,
                    _ => rv_op::illegal,
                },
                _ => rv_op::illegal,
            },
            5 => rv_op::c_j,
            6 => rv_op::c_beqz,
            7 => rv_op::c_bnez,
            _ => rv_op::illegal,
        },
        2 => match (inst >> 13) & 0b111 {
            0 => rv_op::c_slli,
            1 => {
                if isa == rv128 {
                    rv_op::c_lqsp
                } else {
                    rv_op::c_fldsp
                }
            }
            2 => rv_op::c_lwsp,
            3 => {
                if isa == rv32 {
                    rv_op::c_flwsp
                } else {
                    rv_op::c_ldsp
                }
            }
            4 => match (inst >> 12) & 0b1 {
                0 => match (inst >> 2) & 0b11111 {
                    0 => rv_op::c_jr,
                    _ => rv_op::c_mv,
                },
                1 => match (inst >> 2) & 0b11111 {
                    0 => match (inst >> 7) & 0b11111 {
                        0 => rv_op::c_ebreak,
                        _ => rv_op::c_jalr,
                    },
                    _ => rv_op::c_add,
                },
                _ => rv_op::illegal,
            },
            5 => {
                if isa == rv128 {
                    rv_op::c_sqsp
                } else {
                    rv_op::c_fsdsp
                }
            }
            6 => rv_op::c_swsp,
            7 => {
                if isa == rv32 {
                    rv_op::c_fswsp
                } else {
                    rv_op::c_sdsp
                }
            }
            _ => rv_op::illegal,
        },
        3 => match (inst >> 2) & 0b11111 {
            0 => match (inst >> 12) & 0b111 {
                0 => rv_op::lb,
                1 => rv_op::lh,
                2 => rv_op::lw,
                3 => rv_op::ld,
                4 => rv_op::lbu,
                5 => rv_op::lhu,
                6 => rv_op::lwu,
                7 => rv_op::ldu,
                _ => rv_op::illegal,
            },
            1 => match (inst >> 12) & 0b111 {
                2 => rv_op::flw,
                3 => rv_op::fld,
                4 => rv_op::flq,
                _ => rv_op::illegal,
            },
            3 => match (inst >> 12) & 0b111 {
                0 => rv_op::fence,
                1 => rv_op::fence_i,
                2 => rv_op::lq,
                _ => rv_op::illegal,
            },
            4 => match (inst >> 12) & 0b111 {
                0 => rv_op::addi,
                1 => match (inst >> 27) & 0b11111 {
                    0 => rv_op::slli,
                    _ => rv_op::illegal,
                },
                2 => rv_op::slti,
                3 => rv_op::sltiu,
                4 => rv_op::xori,
                5 => match (inst >> 27) & 0b11111 {
                    0 => rv_op::srli,
                    8 => rv_op::srai,
                    _ => rv_op::illegal,
                },
                6 => rv_op::ori,
                7 => rv_op::andi,
                _ => rv_op::illegal,
            },
            5 => rv_op::auipc,
            6 => match (inst >> 12) & 0b111 {
                0 => rv_op::addiw,
                1 => match (inst >> 25) & 0b1111111 {
                    0 => rv_op::slliw,
                    _ => rv_op::illegal,
                },
                5 => match (inst >> 25) & 0b1111111 {
                    0 => rv_op::srliw,
                    32 => rv_op::sraiw,
                    _ => rv_op::illegal,
                },
                _ => rv_op::illegal,
            },
            8 => match (inst >> 12) & 0b111 {
                0 => rv_op::sb,
                1 => rv_op::sh,
                2 => rv_op::sw,
                3 => rv_op::sd,
                4 => rv_op::sq,
                _ => rv_op::illegal,
            },
            9 => match (inst >> 12) & 0b111 {
                2 => rv_op::fsw,
                3 => rv_op::fsd,
                4 => rv_op::fsq,
                _ => rv_op::illegal,
            },
            11 => match ((inst >> 24) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                2 => rv_op::amoadd_w,
                3 => rv_op::amoadd_d,
                4 => rv_op::amoadd_q,
                10 => rv_op::amoswap_w,
                11 => rv_op::amoswap_d,
                12 => rv_op::amoswap_q,
                18 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::lr_w,
                    _ => rv_op::illegal,
                },
                19 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::lr_d,
                    _ => rv_op::illegal,
                },
                20 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::lr_q,
                    _ => rv_op::illegal,
                },
                26 => rv_op::sc_w,
                27 => rv_op::sc_d,
                28 => rv_op::sc_q,
                34 => rv_op::amoxor_w,
                35 => rv_op::amoxor_d,
                36 => rv_op::amoxor_q,
                66 => rv_op::amoor_w,
                67 => rv_op::amoor_d,
                68 => rv_op::amoor_q,
                98 => rv_op::amoand_w,
                99 => rv_op::amoand_d,
                100 => rv_op::amoand_q,
                130 => rv_op::amomin_w,
                131 => rv_op::amomin_d,
                132 => rv_op::amomin_q,
                162 => rv_op::amomax_w,
                163 => rv_op::amomax_d,
                164 => rv_op::amomax_q,
                194 => rv_op::amominu_w,
                195 => rv_op::amominu_d,
                196 => rv_op::amominu_q,
                226 => rv_op::amomaxu_w,
                227 => rv_op::amomaxu_d,
                228 => rv_op::amomaxu_q,
                _ => rv_op::illegal,
            },
            12 => match ((inst >> 22) & 0b1111111000) | ((inst >> 12) & 0b0000000111) {
                0 => rv_op::add,
                1 => rv_op::sll,
                2 => rv_op::slt,
                3 => rv_op::sltu,
                4 => rv_op::xor,
                5 => rv_op::srl,
                6 => rv_op::or,
                7 => rv_op::and,
                8 => rv_op::mul,
                9 => rv_op::mulh,
                10 => rv_op::mulhsu,
                11 => rv_op::mulhu,
                12 => rv_op::div,
                13 => rv_op::divu,
                14 => rv_op::rem,
                15 => rv_op::remu,
                256 => rv_op::sub,
                261 => rv_op::sra,
                _ => rv_op::illegal,
            },
            13 => rv_op::lui,
            14 => match ((inst >> 22) & 0b1111111000) | ((inst >> 12) & 0b0000000111) {
                0 => rv_op::addw,
                1 => rv_op::sllw,
                5 => rv_op::srlw,
                8 => rv_op::mulw,
                12 => rv_op::divw,
                13 => rv_op::divuw,
                14 => rv_op::remw,
                15 => rv_op::remuw,
                256 => rv_op::subw,
                261 => rv_op::sraw,
                _ => rv_op::illegal,
            },
            16 => match (inst >> 25) & 0b11 {
                0 => rv_op::fmadd_s,
                1 => rv_op::fmadd_d,
                3 => rv_op::fmadd_q,
                _ => rv_op::illegal,
            },
            17 => match (inst >> 25) & 0b11 {
                0 => rv_op::fmsub_s,
                1 => rv_op::fmsub_d,
                3 => rv_op::fmsub_q,
                _ => rv_op::illegal,
            },
            18 => match (inst >> 25) & 0b11 {
                0 => rv_op::fnmsub_s,
                1 => rv_op::fnmsub_d,
                3 => rv_op::fnmsub_q,
                _ => rv_op::illegal,
            },
            19 => match (inst >> 25) & 0b11 {
                0 => rv_op::fnmadd_s,
                1 => rv_op::fnmadd_d,
                3 => rv_op::fnmadd_q,
                _ => rv_op::illegal,
            },
            20 => match (inst >> 25) & 0b1111111 {
                0 => rv_op::fadd_s,
                1 => rv_op::fadd_d,
                3 => rv_op::fadd_q,
                4 => rv_op::fsub_s,
                5 => rv_op::fsub_d,
                7 => rv_op::fsub_q,
                8 => rv_op::fmul_s,
                9 => rv_op::fmul_d,
                11 => rv_op::fmul_q,
                12 => rv_op::fdiv_s,
                13 => rv_op::fdiv_d,
                15 => rv_op::fdiv_q,
                16 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fsgnj_s,
                    1 => rv_op::fsgnjn_s,
                    2 => rv_op::fsgnjx_s,
                    _ => rv_op::illegal,
                },
                17 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fsgnj_d,
                    1 => rv_op::fsgnjn_d,
                    2 => rv_op::fsgnjx_d,
                    _ => rv_op::illegal,
                },
                19 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fsgnj_q,
                    1 => rv_op::fsgnjn_q,
                    2 => rv_op::fsgnjx_q,
                    _ => rv_op::illegal,
                },
                20 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fmin_s,
                    1 => rv_op::fmax_s,
                    _ => rv_op::illegal,
                },
                21 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fmin_d,
                    1 => rv_op::fmax_d,
                    _ => rv_op::illegal,
                },
                23 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fmin_q,
                    1 => rv_op::fmax_q,
                    _ => rv_op::illegal,
                },
                32 => match (inst >> 20) & 0b11111 {
                    1 => rv_op::fcvt_s_d,
                    3 => rv_op::fcvt_s_q,
                    _ => rv_op::illegal,
                },
                33 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fcvt_d_s,
                    3 => rv_op::fcvt_d_q,
                    _ => rv_op::illegal,
                },
                35 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fcvt_q_s,
                    1 => rv_op::fcvt_q_d,
                    _ => rv_op::illegal,
                },
                44 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fsqrt_s,
                    _ => rv_op::illegal,
                },
                45 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fsqrt_d,
                    _ => rv_op::illegal,
                },
                47 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fsqrt_q,
                    _ => rv_op::illegal,
                },
                80 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fle_s,
                    1 => rv_op::flt_s,
                    2 => rv_op::feq_s,
                    _ => rv_op::illegal,
                },
                81 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fle_d,
                    1 => rv_op::flt_d,
                    2 => rv_op::feq_d,
                    _ => rv_op::illegal,
                },
                83 => match (inst >> 12) & 0b111 {
                    0 => rv_op::fle_q,
                    1 => rv_op::flt_q,
                    2 => rv_op::feq_q,
                    _ => rv_op::illegal,
                },
                96 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fcvt_w_s,
                    1 => rv_op::fcvt_wu_s,
                    2 => rv_op::fcvt_l_s,
                    3 => rv_op::fcvt_lu_s,
                    _ => rv_op::illegal,
                },
                97 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fcvt_w_d,
                    1 => rv_op::fcvt_wu_d,
                    2 => rv_op::fcvt_l_d,
                    3 => rv_op::fcvt_lu_d,
                    _ => rv_op::illegal,
                },
                99 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fcvt_w_q,
                    1 => rv_op::fcvt_wu_q,
                    2 => rv_op::fcvt_l_q,
                    3 => rv_op::fcvt_lu_q,
                    _ => rv_op::illegal,
                },
                104 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fcvt_s_w,
                    1 => rv_op::fcvt_s_wu,
                    2 => rv_op::fcvt_s_l,
                    3 => rv_op::fcvt_s_lu,
                    _ => rv_op::illegal,
                },
                105 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fcvt_d_w,
                    1 => rv_op::fcvt_d_wu,
                    2 => rv_op::fcvt_d_l,
                    3 => rv_op::fcvt_d_lu,
                    _ => rv_op::illegal,
                },
                107 => match (inst >> 20) & 0b11111 {
                    0 => rv_op::fcvt_q_w,
                    1 => rv_op::fcvt_q_wu,
                    2 => rv_op::fcvt_q_l,
                    3 => rv_op::fcvt_q_lu,
                    _ => rv_op::illegal,
                },
                112 => match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                    0 => rv_op::fmv_x_s,
                    1 => rv_op::fclass_s,
                    _ => rv_op::illegal,
                },
                113 => match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                    0 => rv_op::fmv_x_d,
                    1 => rv_op::fclass_d,
                    _ => rv_op::illegal,
                },
                115 => match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                    0 => rv_op::fmv_x_q,
                    1 => rv_op::fclass_q,
                    _ => rv_op::illegal,
                },
                120 => match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                    0 => rv_op::fmv_s_x,
                    _ => rv_op::illegal,
                },
                121 => match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                    0 => rv_op::fmv_d_x,
                    _ => rv_op::illegal,
                },
                123 => match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                    0 => rv_op::fmv_q_x,
                    _ => rv_op::illegal,
                },
                _ => rv_op::illegal,
            },
            22 => match (inst >> 12) & 0b111 {
                0 => rv_op::addid,
                1 => match (inst >> 26) & 0b111111 {
                    0 => rv_op::sllid,
                    _ => rv_op::illegal,
                },
                5 => match (inst >> 26) & 0b111111 {
                    0 => rv_op::srlid,
                    16 => rv_op::sraid,
                    _ => rv_op::illegal,
                },
                _ => rv_op::illegal,
            },
            24 => match (inst >> 12) & 0b111 {
                0 => rv_op::beq,
                1 => rv_op::bne,
                4 => rv_op::blt,
                5 => rv_op::bge,
                6 => rv_op::bltu,
                7 => rv_op::bgeu,
                _ => rv_op::illegal,
            },
            25 => match (inst >> 12) & 0b111 {
                0 => rv_op::jalr,
                _ => rv_op::illegal,
            },
            27 => rv_op::jal,
            28 => match (inst >> 12) & 0b111 {
                0 => match ((inst >> 20) & 0b111111100000) | ((inst >> 7) & 0b000000011111) {
                    0 => match (inst >> 15) & 0b1111111111 {
                        0 => rv_op::ecall,
                        32 => rv_op::ebreak,
                        64 => rv_op::uret,
                        _ => rv_op::illegal,
                    },
                    256 => match (inst >> 20) & 0b11111 {
                        2 => match (inst >> 15) & 0b11111 {
                            0 => rv_op::sret,
                            _ => rv_op::illegal,
                        },
                        4 => rv_op::sfence_vm,
                        5 => match (inst >> 15) & 0b11111 {
                            0 => rv_op::wfi,
                            _ => rv_op::illegal,
                        },
                        _ => rv_op::illegal,
                    },
                    288 => rv_op::sfence_vma,
                    512 => match (inst >> 15) & 0b1111111111 {
                        64 => rv_op::hret,
                        _ => rv_op::illegal,
                    },
                    768 => match (inst >> 15) & 0b1111111111 {
                        64 => rv_op::mret,
                        _ => rv_op::illegal,
                    },
                    1952 => match (inst >> 15) & 0b1111111111 {
                        576 => rv_op::dret,
                        _ => rv_op::illegal,
                    },
                    _ => rv_op::illegal,
                },
                1 => rv_op::csrrw,
                2 => rv_op::csrrs,
                3 => rv_op::csrrc,
                5 => rv_op::csrrwi,
                6 => rv_op::csrrsi,
                7 => rv_op::csrrci,
                _ => rv_op::illegal,
            },
            30 => match ((inst >> 22) & 0b1111111000) | ((inst >> 12) & 0b0000000111) {
                0 => rv_op::addd,
                1 => rv_op::slld,
                5 => rv_op::srld,
                8 => rv_op::muld,
                12 => rv_op::divd,
                13 => rv_op::divud,
                14 => rv_op::remd,
                15 => rv_op::remud,
                256 => rv_op::subd,
                261 => rv_op::srad,
                _ => rv_op::illegal,
            },
            _ => rv_op::illegal,
        },
        _ => rv_op::illegal,
    }
}

/* operand extractors */

fn operand_rd(inst: rv_inst) -> u8 {
    ((inst << 52) >> 59) as u8
}

fn operand_rs1(inst: rv_inst) -> u8 {
    ((inst << 44) >> 59) as u8
}

fn operand_rs2(inst: rv_inst) -> u8 {
    ((inst << 39) >> 59) as u8
}

fn operand_rs3(inst: rv_inst) -> u8 {
    ((inst << 32) >> 59) as u8
}

fn operand_aq(inst: rv_inst) -> bool {
    ((inst << 37) >> 63) != 0
}

fn operand_rl(inst: rv_inst) -> bool {
    ((inst << 38) >> 63) != 0
}

fn operand_pred(inst: rv_inst) -> u8 {
    ((inst << 36) >> 60) as u8
}

fn operand_succ(inst: rv_inst) -> u8 {
    ((inst << 40) >> 60) as u8
}

fn operand_rm(inst: rv_inst) -> u8 {
    ((inst << 49) >> 61) as u8
}

fn operand_shamt5(inst: rv_inst) -> u32 {
    ((inst << 39) >> 59) as u32
}

fn operand_shamt6(inst: rv_inst) -> u32 {
    ((inst << 38) >> 58) as u32
}

fn operand_shamt7(inst: rv_inst) -> u32 {
    ((inst << 37) >> 57) as u32
}

fn operand_crdq(inst: rv_inst) -> u8 {
    ((inst << 59) >> 61) as u8
}

fn operand_crs1q(inst: rv_inst) -> u8 {
    ((inst << 54) >> 61) as u8
}

fn operand_crs1rdq(inst: rv_inst) -> u8 {
    ((inst << 54) >> 61) as u8
}

fn operand_crs2q(inst: rv_inst) -> u8 {
    ((inst << 59) >> 61) as u8
}

fn operand_crd(inst: rv_inst) -> u8 {
    ((inst << 52) >> 59) as u8
}

fn operand_crs1(inst: rv_inst) -> u8 {
    ((inst << 52) >> 59) as u8
}

fn operand_crs1rd(inst: rv_inst) -> u8 {
    ((inst << 52) >> 59) as u8
}

fn operand_crs2(inst: rv_inst) -> u8 {
    ((inst << 57) >> 59) as u8
}

fn operand_cimmsh5(inst: rv_inst) -> u32 {
    ((inst << 57) >> 59) as u32
}

fn operand_csr12(inst: rv_inst) -> u32 {
    ((inst << 32) >> 52) as u32
}

fn signed(inst: rv_inst) -> i64 {
    inst as i64
}

fn operand_imm12(inst: rv_inst) -> i32 {
    (signed(inst << 32) >> 52) as i32
}

fn operand_imm20(inst: rv_inst) -> i32 {
    ((signed(inst << 32) >> 44) << 12) as i32
}

fn operand_jimm20(inst: rv_inst) -> i32 {
    ((signed(inst << 32) >> 63) << 20
        | signed((inst << 33) >> 54) << 1
        | signed((inst << 43) >> 63) << 11
        | signed((inst << 44) >> 56) << 12) as i32
}

fn operand_simm12(inst: rv_inst) -> i32 {
    ((signed(inst << 32) >> 57) << 5 | signed((inst << 52) >> 59)) as i32
}

fn operand_sbimm12(inst: rv_inst) -> i32 {
    ((signed(inst << 32) >> 63) << 12
        | signed((inst << 33) >> 58) << 5
        | signed((inst << 52) >> 60) << 1
        | signed((inst << 56) >> 63) << 11) as i32
}

fn operand_cimmsh6(inst: rv_inst) -> u32 {
    (((inst << 51) >> 63) << 5 | (inst << 57) >> 59) as u32
}

fn operand_cimmi(inst: rv_inst) -> i32 {
    ((signed(inst << 51) >> 63) << 5 | signed((inst << 57) >> 59)) as i32
}

fn operand_cimmui(inst: rv_inst) -> i32 {
    ((signed(inst << 51) >> 63) << 17 | signed((inst << 57) >> 59) << 12) as i32
}

fn operand_cimmlwsp(inst: rv_inst) -> u32 {
    (((inst << 51) >> 63) << 5 | ((inst << 57) >> 61) << 2 | ((inst << 60) >> 62) << 6) as u32
}

fn operand_cimmldsp(inst: rv_inst) -> u32 {
    (((inst << 51) >> 63) << 5 | ((inst << 57) >> 62) << 3 | ((inst << 59) >> 61) << 6) as u32
}

fn operand_cimmlqsp(inst: rv_inst) -> u32 {
    (((inst << 51) >> 63) << 5 | ((inst << 57) >> 63) << 4 | ((inst << 58) >> 60) << 6) as u32
}

fn operand_cimm16sp(inst: rv_inst) -> i32 {
    ((signed(inst << 51) >> 63) << 9
        | signed((inst << 57) >> 63) << 4
        | signed((inst << 58) >> 63) << 6
        | signed((inst << 59) >> 62) << 7
        | signed((inst << 61) >> 63) << 5) as i32
}

fn operand_cimmj(inst: rv_inst) -> i32 {
    ((signed(inst << 51) >> 63) << 11
        | signed((inst << 52) >> 63) << 4
        | signed((inst << 53) >> 62) << 8
        | signed((inst << 55) >> 63) << 10
        | signed((inst << 56) >> 63) << 6
        | signed((inst << 57) >> 63) << 7
        | signed((inst << 58) >> 61) << 1
        | signed((inst << 61) >> 63) << 5) as i32
}

fn operand_cimmb(inst: rv_inst) -> i32 {
    ((signed(inst << 51) >> 63) << 8
        | signed((inst << 52) >> 62) << 3
        | signed((inst << 57) >> 62) << 6
        | signed((inst << 59) >> 62) << 1
        | signed((inst << 61) >> 63) << 5) as i32
}

fn operand_cimmswsp(inst: rv_inst) -> u32 {
    (((inst << 51) >> 60) << 2 | ((inst << 55) >> 62) << 6) as u32
}

fn operand_cimmsdsp(inst: rv_inst) -> u32 {
    (((inst << 51) >> 61) << 3 | ((inst << 54) >> 61) << 6) as u32
}

fn operand_cimmsqsp(inst: rv_inst) -> u32 {
    (((inst << 51) >> 62) << 4 | ((inst << 53) >> 60) << 6) as u32
}

fn operand_cimm4spn(inst: rv_inst) -> u32 {
    (((inst << 51) >> 62) << 4
        | ((inst << 53) >> 60) << 6
        | ((inst << 57) >> 63) << 2
        | ((inst << 58) >> 63) << 3) as u32
}

fn operand_cimmw(inst: rv_inst) -> u32 {
    (((inst << 51) >> 61) << 3 | ((inst << 57) >> 63) << 2 | ((inst << 58) >> 63) << 6) as u32
}

fn operand_cimmd(inst: rv_inst) -> u32 {
    (((inst << 51) >> 61) << 3 | ((inst << 57) >> 62) << 6) as u32
}

fn operand_cimmq(inst: rv_inst) -> u32 {
    (((inst << 51) >> 62) << 4 | ((inst << 53) >> 63) << 8 | ((inst << 57) >> 62) << 6) as u32
}

/* decode operands */

fn decode_inst_operands(dec: &mut rv_decode) {
    let inst = dec.inst;
    dec.codec = opcode_data[dec.op as usize].codec;
    match dec.codec {
        rv_codec::none => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = 0;
        }
        rv_codec::u => {
            dec.rd = operand_rd(inst);
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_imm20(inst);
        }
        rv_codec::uj => {
            dec.rd = operand_rd(inst);
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_jimm20(inst);
        }
        rv_codec::i => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_imm12(inst);
        }
        rv_codec::i_sh5 => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_shamt5(inst) as i32;
        }
        rv_codec::i_sh6 => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_shamt6(inst) as i32;
        }
        rv_codec::i_sh7 => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_shamt7(inst) as i32;
        }
        rv_codec::i_csr => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_csr12(inst) as i32;
        }
        rv_codec::s => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = operand_rs2(inst);
            dec.imm = operand_simm12(inst);
        }
        rv_codec::sb => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = operand_rs2(inst);
            dec.imm = operand_sbimm12(inst);
        }
        rv_codec::r => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = operand_rs2(inst);
            dec.imm = 0;
        }
        rv_codec::r_m => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = operand_rs2(inst);
            dec.imm = 0;
            dec.rm = operand_rm(inst);
        }
        rv_codec::r4_m => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = operand_rs2(inst);
            dec.rs3 = operand_rs3(inst);
            dec.imm = 0;
            dec.rm = operand_rm(inst);
        }
        rv_codec::r_a => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = operand_rs2(inst);
            dec.imm = 0;
            dec.aq = operand_aq(inst);
            dec.rl = operand_rl(inst);
        }
        rv_codec::r_l => {
            dec.rd = operand_rd(inst);
            dec.rs1 = operand_rs1(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = 0;
            dec.aq = operand_aq(inst);
            dec.rl = operand_rl(inst);
        }
        rv_codec::r_f => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.pred = operand_pred(inst);
            dec.succ = operand_succ(inst);
            dec.imm = 0;
        }
        rv_codec::cb => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = operand_crs1q(inst) + 8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmb(inst);
        }
        rv_codec::cb_imm => {
            dec.rd = operand_crs1rdq(inst) + 8;
            dec.rs1 = operand_crs1rdq(inst) + 8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmi(inst);
        }
        rv_codec::cb_sh5 => {
            dec.rd = operand_crs1rdq(inst) + 8;
            dec.rs1 = operand_crs1rdq(inst) + 8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmsh5(inst) as i32;
        }
        rv_codec::cb_sh6 => {
            dec.rd = operand_crs1rdq(inst) + 8;
            dec.rs1 = operand_crs1rdq(inst) + 8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmsh6(inst) as i32;
        }
        rv_codec::ci => {
            dec.rd = operand_crs1rd(inst);
            dec.rs1 = operand_crs1rd(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmi(inst);
        }
        rv_codec::ci_sh5 => {
            dec.rd = operand_crs1rd(inst);
            dec.rs1 = operand_crs1rd(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmsh5(inst) as i32;
        }
        rv_codec::ci_sh6 => {
            dec.rd = operand_crs1rd(inst);
            dec.rs1 = operand_crs1rd(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmsh6(inst) as i32;
        }
        rv_codec::ci_16sp => {
            dec.rd = rv_ireg::sp as u8;
            dec.rs1 = rv_ireg::sp as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimm16sp(inst);
        }
        rv_codec::ci_lwsp => {
            dec.rd = operand_crd(inst);
            dec.rs1 = rv_ireg::sp as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmlwsp(inst) as i32;
        }
        rv_codec::ci_ldsp => {
            dec.rd = operand_crd(inst);
            dec.rs1 = rv_ireg::sp as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmldsp(inst) as i32;
        }
        rv_codec::ci_lqsp => {
            dec.rd = operand_crd(inst);
            dec.rs1 = rv_ireg::sp as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmlqsp(inst) as i32;
        }
        rv_codec::ci_li => {
            dec.rd = operand_crd(inst);
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmi(inst);
        }
        rv_codec::ci_lui => {
            dec.rd = operand_crd(inst);
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmui(inst);
        }
        rv_codec::ci_none => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = 0;
        }
        rv_codec::ciw_4spn => {
            dec.rd = operand_crdq(inst) + 8;
            dec.rs1 = rv_ireg::sp as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimm4spn(inst) as i32;
        }
        rv_codec::cj => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmj(inst);
        }
        rv_codec::cj_jal => {
            dec.rd = rv_ireg::ra as u8;
            dec.rs1 = rv_ireg::zero as u8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmj(inst);
        }
        rv_codec::cl_lw => {
            dec.rd = operand_crdq(inst) + 8;
            dec.rs1 = operand_crs1q(inst) + 8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmw(inst) as i32;
        }
        rv_codec::cl_ld => {
            dec.rd = operand_crdq(inst) + 8;
            dec.rs1 = operand_crs1q(inst) + 8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmd(inst) as i32;
        }
        rv_codec::cl_lq => {
            dec.rd = operand_crdq(inst) + 8;
            dec.rs1 = operand_crs1q(inst) + 8;
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = operand_cimmq(inst) as i32;
        }
        rv_codec::cr => {
            dec.rd = operand_crs1rd(inst);
            dec.rs1 = operand_crs1rd(inst);
            dec.rs2 = operand_crs2(inst);
            dec.imm = 0;
        }
        rv_codec::cr_mv => {
            dec.rd = operand_crd(inst);
            dec.rs1 = operand_crs2(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = 0;
        }
        rv_codec::cr_jalr => {
            dec.rd = rv_ireg::ra as u8;
            dec.rs1 = operand_crs1(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = 0;
        }
        rv_codec::cr_jr => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = operand_crs1(inst);
            dec.rs2 = rv_ireg::zero as u8;
            dec.imm = 0;
        }
        rv_codec::cs => {
            dec.rd = operand_crs1rdq(inst) + 8;
            dec.rs1 = operand_crs1rdq(inst) + 8;
            dec.rs2 = operand_crs2q(inst) + 8;
            dec.imm = 0;
        }
        rv_codec::cs_sw => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = operand_crs1q(inst) + 8;
            dec.rs2 = operand_crs2q(inst) + 8;
            dec.imm = operand_cimmw(inst) as i32;
        }
        rv_codec::cs_sd => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = operand_crs1q(inst) + 8;
            dec.rs2 = operand_crs2q(inst) + 8;
            dec.imm = operand_cimmd(inst) as i32;
        }
        rv_codec::cs_sq => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = operand_crs1q(inst) + 8;
            dec.rs2 = operand_crs2q(inst) + 8;
            dec.imm = operand_cimmq(inst) as i32;
        }
        rv_codec::css_swsp => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = rv_ireg::sp as u8;
            dec.rs2 = operand_crs2(inst);
            dec.imm = operand_cimmswsp(inst) as i32;
        }
        rv_codec::css_sdsp => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = rv_ireg::sp as u8;
            dec.rs2 = operand_crs2(inst);
            dec.imm = operand_cimmsdsp(inst) as i32;
        }
        rv_codec::css_sqsp => {
            dec.rd = rv_ireg::zero as u8;
            dec.rs1 = rv_ireg::sp as u8;
            dec.rs2 = operand_crs2(inst);
            dec.imm = operand_cimmsqsp(inst) as i32;
        }
        rv_codec::illegal => {}
    };
}

/* check constraint */

fn check_constraints(dec: &rv_decode, constraint: &[rvc_constraint]) -> bool {
    let imm = dec.imm;
    let rd = dec.rd;
    let rs1 = dec.rs1;
    let rs2 = dec.rs2;
    for c in constraint {
        match c {
            rvc_constraint::rd_eq_ra => {
                if !(rd == 1) {
                    return false;
                }
            }
            rvc_constraint::rd_eq_x0 => {
                if !(rd == 0) {
                    return false;
                }
            }
            rvc_constraint::rs1_eq_x0 => {
                if !(rs1 == 0) {
                    return false;
                }
            }
            rvc_constraint::rs2_eq_x0 => {
                if !(rs2 == 0) {
                    return false;
                }
            }
            rvc_constraint::rs2_eq_rs1 => {
                if !(rs2 == rs1) {
                    return false;
                }
            }
            rvc_constraint::rs1_eq_ra => {
                if !(rs1 == 1) {
                    return false;
                }
            }
            rvc_constraint::imm_eq_zero => {
                if !(imm == 0) {
                    return false;
                }
            }
            rvc_constraint::imm_eq_n1 => {
                if !(imm == -1) {
                    return false;
                }
            }
            rvc_constraint::imm_eq_p1 => {
                if !(imm == 1) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0x001 => {
                if !(imm == 0x001) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0x002 => {
                if !(imm == 0x002) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0x003 => {
                if !(imm == 0x003) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0xc00 => {
                if !(imm == 0xc00) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0xc01 => {
                if !(imm == 0xc01) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0xc02 => {
                if !(imm == 0xc02) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0xc80 => {
                if !(imm == 0xc80) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0xc81 => {
                if !(imm == 0xc81) {
                    return false;
                }
            }
            rvc_constraint::csr_eq_0xc82 => {
                if !(imm == 0xc82) {
                    return false;
                }
            }
        }
    }
    return true;
}

/* instruction length */

pub fn inst_length(inst: rv_inst) -> usize {
    /* NOTE: supports maximum instruction size of 64-bits */

    /* instruction length coding
     *
     *      aa - 16 bit aa != 11
     *   bbb11 - 32 bit bbb != 111
     *  011111 - 48 bit
     * 0111111 - 64 bit
     */

    if (inst & 0b11) != 0b11 {
        2
    } else if (inst & 0b11100) != 0b11100 {
        4
    } else if (inst & 0b111111) == 0b011111 {
        6
    } else if (inst & 0b1111111) == 0b0111111 {
        8
    } else {
        0
    }
}

/* lift instruction to pseudo-instruction */

fn decode_inst_lift_pseudo(dec: &mut rv_decode) {
    for comp_data in opcode_data[dec.op as usize].pseudo {
        if check_constraints(dec, comp_data.constraints) {
            dec.op = comp_data.op;
            dec.codec = opcode_data[dec.op as usize].codec;
            return;
        }
    }
}

/* decompress instruction */

fn decompress_inst_rv32(dec: &mut rv_decode) {
    let decomp_op = opcode_data[dec.op as usize].decomp_rv32;
    if decomp_op != rv_op::illegal {
        if (opcode_data[dec.op as usize].decomp_data & rvcd::imm_nz) != 0 && dec.imm == 0 {
            dec.op = rv_op::illegal;
        } else {
            dec.op = decomp_op;
            dec.codec = opcode_data[decomp_op as usize].codec;
        }
    }
}

fn decompress_inst_rv64(dec: &mut rv_decode) {
    let decomp_op = opcode_data[dec.op as usize].decomp_rv64;
    if decomp_op != rv_op::illegal {
        if (opcode_data[dec.op as usize].decomp_data & rvcd::imm_nz) != 0 && dec.imm == 0 {
            dec.op = rv_op::illegal;
        } else {
            dec.op = decomp_op;
            dec.codec = opcode_data[decomp_op as usize].codec;
        }
    }
}

fn decompress_inst_rv128(dec: &mut rv_decode) {
    let decomp_op = opcode_data[dec.op as usize].decomp_rv128;
    if decomp_op != rv_op::illegal {
        if (opcode_data[dec.op as usize].decomp_data & rvcd::imm_nz) != 0 && dec.imm == 0 {
            dec.op = rv_op::illegal;
        } else {
            dec.op = decomp_op;
            dec.codec = opcode_data[decomp_op as usize].codec;
        }
    }
}

/* disassemble instruction */

pub fn decode_inst(isa: rv_isa, pc: u64, inst: rv_inst) -> rv_decode {
    let mut dec: rv_decode = rv_decode {
        pc: pc,
        inst: inst,
        imm: 0,
        op: rv_op::illegal,
        codec: rv_codec::illegal,
        rd: 0,
        rs1: 0,
        rs2: 0,
        rs3: 0,
        rm: 0,
        pred: 0,
        succ: 0,
        aq: false,
        rl: false,
    };
    dec.pc = pc;
    dec.inst = inst;
    dec.op = decode_inst_opcode(dec.inst, isa);
    decode_inst_operands(&mut dec);
    match isa {
        rv_isa::rv32 => decompress_inst_rv32(&mut dec),
        rv_isa::rv64 => decompress_inst_rv64(&mut dec),
        rv_isa::rv128 => decompress_inst_rv128(&mut dec),
    }
    decode_inst_lift_pseudo(&mut dec);
    dec
}
