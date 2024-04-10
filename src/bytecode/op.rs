#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, strum::Display, strum::FromRepr,
)]
#[repr(u8)]
pub enum Op {
    Return = 0x00,
    Pop = 0x01,
    PopN(u8) = 0x02,

    Constant(u8) = 0x03,
    Nil = 0x04,
    False = 0x05,
    True = 0x06,

    Call(u8) = 0x10,
    Index = 0x11,
    Func(u8) = 0x12,

    LocalGet(u8) = 0x20,
    LocalSet(u8) = 0x21,
    GlobalDefine(u8) = 0x22,
    GlobalGet(u8) = 0x23,
    GlobalSet(u8) = 0x24,

    Not = 0x30,
    Eq = 0x31,
    Ne = 0x32,
    Gt = 0x33,
    Ge = 0x34,
    Lt = 0x35,
    Le = 0x36,

    Neg = 0x37,
    Add = 0x38,
    Sub = 0x39,
    Mul = 0x3a,
    Div = 0x3b,
    Rem = 0x3c,

    // Jumps
    Jump(i16) = 0x60,
    JumpFalsePeek(i16) = 0x61,
    JumpFalsePop(i16) = 0x62,
    JumpTruePeek(i16) = 0x63,
    JumpTruePop(i16) = 0x64,
}

#[derive(thiserror::Error, Debug)]
pub enum OpError {
    #[error("unknown opcode: {:x}", opcode)]
    UnknownOpcode { opcode: u8 },

    #[error("missing byte for op: {:?}, byte {}", op, b)]
    MissingByte { op: Op, b: usize },
}

impl Op {
    pub fn scan(code: &[u8]) -> Result<Option<Self>, OpError> {
        let Some(&opcode) = code.first() else {
            return Ok(None);
        };

        let op = Self::from_repr(opcode).ok_or(OpError::UnknownOpcode { opcode })?;

        let mut build = op;
        match build {
            Op::Return
            | Op::Pop
            | Op::Nil
            | Op::False
            | Op::True
            | Op::Index
            | Op::Not
            | Op::Eq
            | Op::Ne
            | Op::Gt
            | Op::Ge
            | Op::Lt
            | Op::Le
            | Op::Neg
            | Op::Add
            | Op::Sub
            | Op::Mul
            | Op::Div
            | Op::Rem => {}

            Op::Constant(ref mut byte)
            | Op::PopN(ref mut byte)
            | Op::Call(ref mut byte)
            | Op::Func(ref mut byte)
            | Op::LocalGet(ref mut byte)
            | Op::LocalSet(ref mut byte)
            | Op::GlobalDefine(ref mut byte)
            | Op::GlobalGet(ref mut byte)
            | Op::GlobalSet(ref mut byte) => {
                *byte = *code.get(1).ok_or(OpError::MissingByte { op, b: 1 })?;
            }

            Op::Jump(ref mut int)
            | Op::JumpFalsePeek(ref mut int)
            | Op::JumpFalsePop(ref mut int)
            | Op::JumpTruePeek(ref mut int)
            | Op::JumpTruePop(ref mut int) => {
                let hi = code.get(1).ok_or(OpError::MissingByte { op, b: 1 })?;
                let lo = code.get(2).ok_or(OpError::MissingByte { op, b: 2 })?;
                *int = i16::from_be_bytes([*hi, *lo]);
            }
        }

        Ok(Some(build))
    }

    pub fn size_bytes(&self) -> usize {
        self.to_bytes().len()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Safety: This is the recommended way in The Book for getting the discriminant value
        // when the enums hold values.
        //
        // https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
        let opcode = unsafe { *(self as *const Self as *const u8) };

        let mut bytes = vec![opcode];

        match self {
            Op::Return
            | Op::Pop
            | Op::Nil
            | Op::False
            | Op::True
            | Op::Index
            | Op::Not
            | Op::Eq
            | Op::Ne
            | Op::Gt
            | Op::Ge
            | Op::Lt
            | Op::Le
            | Op::Neg
            | Op::Add
            | Op::Sub
            | Op::Mul
            | Op::Div
            | Op::Rem => {}

            Op::Constant(byte)
            | Op::PopN(byte)
            | Op::Call(byte)
            | Op::Func(byte)
            | Op::LocalGet(byte)
            | Op::LocalSet(byte)
            | Op::GlobalDefine(byte)
            | Op::GlobalGet(byte)
            | Op::GlobalSet(byte) => {
                bytes.push(*byte);
            }

            Op::Jump(int)
            | Op::JumpFalsePeek(int)
            | Op::JumpFalsePop(int)
            | Op::JumpTruePeek(int)
            | Op::JumpTruePop(int) => {
                bytes.extend(int.to_be_bytes());
            }
        }

        bytes
    }
}
