use std::cmp;

use disasm::{decode_inst, inst_length};
use types::*;

#[derive(Debug, Clone)]
pub struct Disassembler<'a> {
    isa: rv_isa,
    slice: &'a [u8],
    ptr: usize,
    pc: u64,
}

impl<'a> Disassembler<'a> {
    pub fn new(isa: rv_isa, slice: &[u8], start_pc: u64) -> Disassembler {
        return Disassembler {
            isa: isa,
            slice: slice,
            ptr: 0,
            pc: start_pc,
        };
    }
}

impl<'a> Iterator for Disassembler<'a> {
    type Item = rv_decode;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inst =
            (*self.slice.get(self.ptr)? as u64) | ((*self.slice.get(self.ptr + 1)? as u64) << 8);
        let len = cmp::max(inst_length(inst), 2);
        for x in 2..len {
            inst |= (*self.slice.get(self.ptr + x)? as u64) << (x * 8);
        }
        let decoded = Some(decode_inst(self.isa, self.pc, inst));
        self.ptr += len;
        self.pc += len as u64;
        decoded
    }
}
