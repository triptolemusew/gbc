use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    NOP,
    LD,
    LDD,
    LDI,
    LDH,
    LDHL,
    ADD,
    SUB,
    XOR,
    CP,
    INC,
    DEC,
    JP,
    JR,
    CALL,
    RET,
    RST,
    PUSH,
    RL,
    RLA,
    POP,
    PREFIX,
    BIT,
    DI,
    OR,
    EI,
    CPL,
    AND,
    SWAP,
    RES,
    RETI,
    SLA,
}

#[derive(Debug, Clone)]
pub struct Opcode {
    pub code: u8,
    pub mnemonic: String,
    pub operation: Operation,
    pub length: u8,
    pub cycles: u8,
}

impl Opcode {
    fn new(code: u8, mnemonic: String, operation: Operation, length: u8, cycles: u8) -> Self {
        Opcode {
            code,
            mnemonic,
            operation,
            length,
            cycles,
        }
    }
}


lazy_static! {
    pub static ref OPCODES: Vec<Opcode> = vec![];
}
