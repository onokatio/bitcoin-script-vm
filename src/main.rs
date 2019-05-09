use std::io;

enum OPCODE {
    OP_1,
}

struct VM {
    stack: Vec<i32>,
    code: Vec<OPCODE>,
    pc: usize,
}

impl VM {
    fn new(code: Vec<OPCODE>) -> VM {
        VM {
            code: code,
            pc: 0,
            stack: vec![]
        }
    }
    fn run(&mut self) {
        match self.code[self.pc] {
            OPCODE::OP_1 => self.op_1(),
        }
    }
    fn op_1(&mut self){
        self.stack.push(1);
        self.pc += 1;
    }
}

fn main() {
    let mut vm = VM::new(vec![OPCODE::OP_1]);
    vm.run();
    println!("pc:{} stack:{}", vm.pc, vm.stack[0]);
}
