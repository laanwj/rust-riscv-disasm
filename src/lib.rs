#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
// Coding conventions
#![deny(missing_debug_implementations)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//#![deny(unused_mut)]
//#![warn(missing_docs)]

mod disasm;
mod types;
mod opcode_data;

pub use types::{rv_isa,rv_rm,rv_fence,rv_ireg,rv_freg,rv_codec,rv_op,rv_decode};
pub use disasm::{disasm_inst, inst_length, decode_inst, format_inst};
