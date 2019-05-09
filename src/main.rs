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
        opcode_list.insert("OP_0",     0x00);
        opcode_list.insert("OP_1",     0x01);
        opcode_list.insert("OP_2",     0x02);
        opcode_list.insert("OP_DUP",   0x76);

        let mut opcode_alias_list: BiMap<&'static str, &'static str> = BiMap::new();
        opcode_alias_list.insert("OP_FALSE", "OP_0");
        opcode_alias_list.insert("OP_TRUE",  "OP_1");

        return Compiler {
            opcode_list: opcode_list,
            opcode_alias_list: opcode_alias_list,
        };
    }
    fn compile(&self, codes: Vec<&str>) -> Vec<i32> {
        let mut codes_unalias: Vec<&str> = vec![];

        for code in codes {
            let alias_opcode: &str = match self.opcode_alias_list.get_by_left(&code) {
                Some(&value) => value,
                None => code,
            };
            codes_unalias.push(alias_opcode);
        }

        let mut bytecode: Vec<i32> = vec![];

        for code in codes_unalias {
            let hex: i32 = match self.opcode_list.get_by_left(&code) {
                Some(&value) => value,
                None => panic!("[Compiler] opcode not found."),
            };
            bytecode.push(hex);
        }

        return bytecode;
    }

}
struct VM {
    stack: Vec<i32>,
    code: Vec<i32>,
    pc: usize,
}

impl VM {
    fn new(code: Vec<i32>) -> VM {
        VM {
            code: code,
            pc: 0,
            stack: vec![]
        }
    }
    fn run(&mut self) {
        while self.code.len() > self.pc {
            match self.code[self.pc] {
                0x01 => self.op_pushnumber(1),
                0x02 => self.op_pushnumber(2),
                0x76 => self.op_dup(),
                _ => panic!("The opcode is not implemented yet,"),
            }
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
}

fn main() {

    let compiler = Compiler::new();
    let bytecode = compiler.compile(vec!["OP_1", "OP_2", "OP_DUP"]);

    let mut vm = VM::new(bytecode);
    vm.run();

    println!("pc:{}", vm.pc);
    print!("stack: [");
    for value in vm.stack {
        print!("{}, ",value);
    }
    println!("]");
}
