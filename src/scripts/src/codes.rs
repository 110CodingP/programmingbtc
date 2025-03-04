pub enum OpCode {
    Op0,
    OpFalse,
    Op1,
    Op16,
    OpDup,
    OpAdd,
    OpCheckSig,
    OpHash256,
    OpHash160
}

impl OpCode {
    pub fn from_u8(code: u8) -> Option<OpCode> {
        match code {
            0x00 => Some(OpCode::Op0),
            0x76 => Some(OpCode::OpDup),
            0x8b => Some(OpCode::OpAdd),
            0xac => Some(OpCode::OpCheckSig),
            0xaa => Some(OpCode::OpHash256),
            0xa9 => Some(OpCode::OpHash160),
            _ => None
        }
    }
}
