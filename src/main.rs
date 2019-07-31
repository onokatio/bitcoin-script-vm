extern crate bimap;
//extern crate sha2;
//extern crate ripemd160;
//extern crate digest;

//use std::io;
//use std::mem;
//use digest::Digest;
//use sha2::Sha256;
//use ripemd160::Ripemd160;

mod compiler;
use crate::compiler::*;

struct VM<'borrow_code_lifetime> {
    stack: &'borrow_code_lifetime mut Vec<i32>,
    codes: &'borrow_code_lifetime Vec<i32>,
    pc: usize,
}

impl<'borrow_code_lifetime> VM<'borrow_code_lifetime> {
    fn new(codes: &'borrow_code_lifetime Vec<i32>, stack: &'borrow_code_lifetime mut Vec<i32>, pc: usize) -> VM<'borrow_code_lifetime>{
        VM {
            codes: codes,
            stack: stack,
            pc: pc,
        }
    }
    fn dump(&self) {
        println!("pc:{}", self.pc);
        print!("stack: [");
        for value in &mut self.stack.iter() {
            print!("{:#x}({}), ", value,value);
        }
        println!("]");

        print!("codes: [");
        for code in Compiler::new().uncompile(&self.codes) {
            print!("{}, ",code);
        }
        println!("]");
    }
    fn run(&mut self) {
        while self.codes.len() > self.pc {
            let halt = self.step();
            if halt == 1 {
                break;
            }
        }
        //self.dump();
    }
    fn step(&mut self) -> i32{

        let compiler = Compiler::new();
        if      self.codes[self.pc] == compiler.compile_single("OP_0")       { self.op_pushnumber(0); }
        else if self.codes[self.pc] == compiler.compile_single("OP_1NEGATE") { self.op_pushnumber(-1); }
        else if self.codes[self.pc] == compiler.compile_single("OP_1")       { self.op_pushnumber(1); }
        else if self.codes[self.pc] == compiler.compile_single("OP_2")       { self.op_pushnumber(2); }
        else if self.codes[self.pc] == compiler.compile_single("OP_3")       { self.op_pushnumber(3); }
        else if self.codes[self.pc] == compiler.compile_single("OP_4")       { self.op_pushnumber(4); }
        else if self.codes[self.pc] == compiler.compile_single("OP_5")       { self.op_pushnumber(5); }
        else if self.codes[self.pc] == compiler.compile_single("OP_6")       { self.op_pushnumber(6); }
        else if self.codes[self.pc] == compiler.compile_single("OP_7")       { self.op_pushnumber(7); }
        else if self.codes[self.pc] == compiler.compile_single("OP_8")       { self.op_pushnumber(8); }
        else if self.codes[self.pc] == compiler.compile_single("OP_9")       { self.op_pushnumber(9); }
        else if self.codes[self.pc] == compiler.compile_single("OP_10")      { self.op_pushnumber(10); }
        else if self.codes[self.pc] == compiler.compile_single("OP_11")      { self.op_pushnumber(11); }
        else if self.codes[self.pc] == compiler.compile_single("OP_12")      { self.op_pushnumber(12); }
        else if self.codes[self.pc] == compiler.compile_single("OP_13")      { self.op_pushnumber(13); }
        else if self.codes[self.pc] == compiler.compile_single("OP_14")      { self.op_pushnumber(14); }
        else if self.codes[self.pc] == compiler.compile_single("OP_15")      { self.op_pushnumber(15); }
        else if self.codes[self.pc] == compiler.compile_single("OP_16")      { self.op_pushnumber(16); }
        else if self.codes[self.pc] == compiler.compile_single("OP_NOP")     { self.op_nop(); }
        else if self.codes[self.pc] == compiler.compile_single("OP_DUP")     { self.op_dup(); }
        else if self.codes[self.pc] == compiler.compile_single("OP_IF")      { self.op_if(); }
        else if self.codes[self.pc] == compiler.compile_single("OP_ENDIF")   { return 1; }
        else if self.codes[self.pc] == compiler.compile_single("OP_ELSE")    { return 1; }
        else if self.codes[self.pc] == compiler.compile_single("OP_HASH160") { self.op_hash160(); }
        else { panic!("[VM] The opcode {:#x} is not implemented yet,", self.codes[self.pc]); }

        return 0;
    }
    fn run_nothing(&mut self){
        loop {
            let compiler = Compiler::new();
            if self.codes[self.pc] == compiler.compile_single("OP_IF")   { self.op_if(); }
            else if self.codes[self.pc] == compiler.compile_single("OP_ENDIF"){ break; }
            else if self.codes[self.pc] == compiler.compile_single("OP_ELSE"){ break; }
            else { self.pc += 1; }
        }
    }
    fn op_pushnumber(&mut self, num: i32){
        self.stack.push(num);
        self.pc += 1;
    }
    fn op_dup(&mut self){

        let num: i32 = match self.stack.pop() {
            Some(num) => num,
            None => panic!("stack is empty."),
        };

        self.stack.push(num);
        self.stack.push(num);
        self.pc += 1;
    }
    fn op_nop(&mut self){
        self.pc += 1;
    }
    fn op_if(&mut self){

        let bool = self.stack.pop();
        if bool.unwrap() != 0 { // run from OP_IF to OP_ELSE

            // run from next to OP_IF until OP_ELSE or OP_END
            let mut if_vm = VM::new(self.codes, self.stack, self.pc + 1);
            if_vm.run();

            // check weither OP_ELSE or OP_END
            if if_vm.codes[if_vm.pc] == Compiler::new().compile_single("OP_ELSE") {
                if_vm.pc += 1; // first opcode next to OP_ELSE
                if_vm.run_nothing(); // skip from OP_ELSE to OP_ENDIF
            }
            self.pc = if_vm.pc + 1; // get pc at next to OP_END or OP_ELSE

        } else { // run from OP_ELSE to OP_END

            // skip from next to OP_IF until OP_ELSE or OP_END
            let mut if_vm = VM::new(self.codes, self.stack, self.pc + 1);
            if_vm.run_nothing();

            // check weither OP_ELSE or OP_END
            if if_vm.codes[if_vm.pc] == Compiler::new().compile_single("OP_ELSE") {
                if_vm.pc += 1; // first opcode next to OP_ELSE
                if_vm.run(); // skip from OP_IF to OP_ELSE
            }
            self.pc = if_vm.pc + 1; // get pc at next to OP_END or OP_ELSE
        }

        //panic!("debug");
    }
    fn op_hash160(&mut self){

        //let value = self.stack.pop().unwrap();

        //let sha256hash = Sha256::digest(&value.to_be_bytes());
        //let ripemd160hash = Ripemd160::digest(sha256hash.as_slice());


        //self.stack.push(ripemd160hash.as_slice());
    }
}

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
