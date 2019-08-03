extern crate bimap;
//extern crate sha2;
//extern crate ripemd160;
//extern crate digest;

//use std::io;
//use std::mem;
//use digest::Digest;
//use sha2::Sha256;
//use ripemd160::Ripemd160;

pub mod compiler;
pub mod vm;

use crate::compiler::*;
use crate::vm::*;

fn main() {

    let compiler = Compiler::new();
    let bytecode = compiler.compile(vec![
                                    "OP_1",
                                    "OP_IF",
                                        "OP_0",
                                        "OP_IF",
                                            "OP_2",
                                        "OP_ELSE",
                                            "OP_3",
                                        "OP_ENDIF",
                                    "OP_ELSE",
                                        "OP_4",
                                    "OP_ENDIF",
                                    "OP_1NEGATE"]);

    let mut stack: Vec<i32> = vec![];
    let mut vm = VM::new(&bytecode, &mut stack,0);
    vm.dump();
    vm.run();
    vm.dump();

}
