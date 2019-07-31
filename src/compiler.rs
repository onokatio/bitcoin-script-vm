use bimap::BiMap;

pub struct Compiler {
    opcode_list: BiMap<&'static str, i32>,
    opcode_alias_list: BiMap<&'static str, &'static str>,
}

impl Compiler {
    pub fn new() -> Compiler {
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
        opcode_list.insert("OP_HASH160", 0xa9);

        let mut opcode_alias_list: BiMap<&'static str, &'static str> = BiMap::new();
        opcode_alias_list.insert("OP_FALSE", "OP_0");
        opcode_alias_list.insert("OP_TRUE",  "OP_1");

        return Compiler {
            opcode_list: opcode_list,
            opcode_alias_list: opcode_alias_list,
        };
    }
    pub fn compile_single(&self, code: &str) -> i32 {
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
    pub fn compile(&self, codes: Vec<&str>) -> Vec<i32> {
        let mut bytecode: Vec<i32> = vec![];

        for code in codes {
            let hex = self.compile_single(code);
            bytecode.push(hex);
        }

        return bytecode;
    }
   pub fn uncompile_single(&self, hex: &i32) -> &str {
        let code: &str = match self.opcode_list.get_by_right(&hex) {
            Some(&value) => value,
            None => panic!("[Compiler] opcode not found."),
        };
        return code;
    }
    pub fn uncompile(&self, hexs: &Vec<i32>) -> Vec<&str> {
        let mut codes: Vec<&str> = vec![];

        for hex in hexs {
            let code = self.uncompile_single(&hex);
            codes.push(code);
        }

        return codes;
    }

}
