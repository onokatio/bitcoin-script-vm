extern crate bimap;

//use std::io;
use bimap::BiMap;

struct Compiler {
    opcode_list: BiMap<&'static str, i32>,
}

impl Compiler {
    fn new() {
        let mut opcode_list: BiMap<&'static str, i32> = BiMap::new();
        opcode_list.insert("OP_0",     0x00);
        opcode_list.insert("OP_FALSE", 0x00);
        opcode_list.insert("OP_1",     0x01);
        opcode_list.insert("OP_TRUE",  0x01);
        opcode_list.insert("OP_2",     0x02);
        opcode_list.insert("OP_DUP",   0x76);

        Compiler {
            opcode_list: opcode_list,
        };
    }
    fn compile(&self, codes:Vec<&str>) -> Vec<i32> {
        let mut bytecode: Vec<i32> = vec![];

        for code in codes {
            let hex: i32 = match self.opcode_list.get_by_left(&code) {
                Some(&value) => value,
                None => panic!("opcode not implemented."),
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
            println!("pc = {}",self.pc);

            match self.code[self.pc] {
                //0x01 => self.op_1(),
                //OP_DUP => self.op_dup(),
                _ => panic!("The opcode is not implemented yet,"),
            }
        }
    }
    fn op_1(&mut self){
        self.stack.push(1);
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
    let mut vm = VM::new(vec![]);
    vm.run();
    println!("pc:{}", vm.pc);

    for op in vm.code {
        println!("{:?}",op);
    }

    print!("stack: [");
    for value in vm.stack {
        print!("{}, ",value);
    }
    println!("]");

    let compiler = Compiler::new();
}
