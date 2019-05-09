use std::io;

enum OPCODE {
    OP_1,
    OP_2,
    OP_DUP,
    OP_IF,
}

struct VM {
    stack: Vec<i32>,
    code: Vec<OPCODE>,
    pc: i32,
}

impl VM {
    fn new(code: Vec<OPCODE>) -> VM {
        VM {
            code: code,
            pc: 0,
            stack: vec![0]
        }
    }
}

fn main() {
}
