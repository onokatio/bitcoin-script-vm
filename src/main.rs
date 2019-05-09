use std::io;
use std::collections::HashMap;

const OP_0:   i32 = 0x00;
const OP_1:   i32 = 0x01;
const OP_2:   i32 = 0x01;
const OP_DUP: i32 = 0x76;

fn string2bytecode(code: Vec<String>) {
    let mut opcode: HashMap<&str, i32> = HashMap::new();
    opcode.insert("OP_0",     0x00);
    opcode.insert("OP_FALSE", 0);
    opcode.insert("OP_1",     1);
    opcode.insert("OP_TRUE",  1);
    opcode.insert("OP_2",     2);

    let mut bytecode: HashMap<&str, i32> = HashMap::new();
    for key in opcode.keys() {
        println!("{}",key);
        match opcode.get(key) {
            Some(value) => println!("{}",value),
            _ => panic!("opcode map is invalid."),
        }
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
                OP_1 => self.op_1(),
                OP_DUP => self.op_dup(),
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
    let mut vm = VM::new(vec![OP_1, OP_DUP]);
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

    string2bytecode(vec![]);
}
