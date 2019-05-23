extern crate bimap;

//use std::io;
use bimap::BiMap;

struct Compiler {
    opcode_list: BiMap<&'static str, i32>,
    opcode_alias_list: BiMap<&'static str, &'static str>,
}

impl Compiler {
    fn new() -> Compiler {
        let mut opcode_list: BiMap<&'static str, i32> = BiMap::new();
        opcode_list.insert("OP_0",       0x00);
        opcode_list.insert("OP_1NEGATE", 0x4f);
        opcode_list.insert("OP_1",       0x51);
        opcode_list.insert("OP_2",       0x52);
        opcode_list.insert("OP_3",       0x53);
        opcode_list.insert("OP_4",       0x54);
        opcode_list.insert("OP_5",       0x55);
        opcode_list.insert("OP_6",       0x56);
        opcode_list.insert("OP_7",       0x57);
        opcode_list.insert("OP_8",       0x58);
        opcode_list.insert("OP_9",       0x59);
        opcode_list.insert("OP_10",      0x5a);
        opcode_list.insert("OP_11",      0x5b);
        opcode_list.insert("OP_12",      0x5c);
        opcode_list.insert("OP_13",      0x5d);
        opcode_list.insert("OP_14",      0x5e);
        opcode_list.insert("OP_15",      0x5f);
        opcode_list.insert("OP_16",      0x60);
        opcode_list.insert("OP_NOP",     0x61);
        opcode_list.insert("OP_DUP",     0x76);
        opcode_list.insert("OP_IF",      0x63);
        opcode_list.insert("OP_NOTIF",   0x64);
        opcode_list.insert("OP_ELSE",    0x67);
        opcode_list.insert("OP_ENDIF",   0x68);

        let mut opcode_alias_list: BiMap<&'static str, &'static str> = BiMap::new();
        opcode_alias_list.insert("OP_FALSE", "OP_0");
        opcode_alias_list.insert("OP_TRUE",  "OP_1");

        return Compiler {
            opcode_list: opcode_list,
            opcode_alias_list: opcode_alias_list,
        };
    }
    fn compile_single(&self, code: &str) -> i32 {
        let alias_opcode: &str = match self.opcode_alias_list.get_by_left(&code) {
            Some(&value) => value,
            None => code,
        };
        let hex: i32 = match self.opcode_list.get_by_left(&alias_opcode) {
            Some(&value) => value,
            None => panic!("[Compiler] opcode not found."),
        };
        return hex;
    }
    fn compile(&self, codes: Vec<&str>) -> Vec<i32> {
        let mut bytecode: Vec<i32> = vec![];

        for code in codes {
            let hex = self.compile_single(code);
            bytecode.push(hex);
        }

        return bytecode;
    }
    fn uncompile_single(&self, hex: &i32) -> &str {
        let code: &str = match self.opcode_list.get_by_right(&hex) {
            Some(&value) => value,
            None => panic!("[Compiler] opcode not found."),
        };
        return code;
    }
    fn uncompile(&self, hexs: &Vec<i32>) -> Vec<&str> {
        let mut codes: Vec<&str> = vec![];

        for hex in hexs {
            let code = self.uncompile_single(&hex);
            codes.push(code);
        }

        return codes;
    }

}
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
            print!("{:#x}, ", value);
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
        self.dump();
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

}
