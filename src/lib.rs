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

/* functions */

//size_t inst_length(rv_inst inst);
//void disasm_inst(char *buf, size_t buflen, rv_isa isa, u64 pc, rv_inst inst);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
