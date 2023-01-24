use crate::value::{Value, ValueArray};

#[repr(u8)]
pub enum OpCode {
    Constant,
    Nil,
    True,
    False,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Return,
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value as u8
    }
}

impl TryInto<OpCode> for u8 {
    type Error = ();

    fn try_into(self) -> Result<OpCode, Self::Error> {
        use OpCode::*;
        match self {
            x if x == Constant as u8 => Ok(Constant),
            x if x == Nil as u8 => Ok(Nil),
            x if x == True as u8 => Ok(True),
            x if x == False as u8 => Ok(False),
            x if x == Equal as u8 => Ok(Equal),
            x if x == Greater as u8 => Ok(Greater),
            x if x == Less as u8 => Ok(Less),
            x if x == Add as u8 => Ok(Add),
            x if x == Subtract as u8 => Ok(Subtract),
            x if x == Multiply as u8 => Ok(Multiply),
            x if x == Divide as u8 => Ok(Divide),
            x if x == Not as u8 => Ok(Not),
            x if x == Negate as u8 => Ok(Negate),
            x if x == Return as u8 => Ok(Return),
            _ => Err(()),
        }
    }
}

pub struct Chunk {
    pub(crate) code: Vec<u8>,
    pub(crate) constants: ValueArray,
    pub(crate) lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: impl Into<u8>, line: usize) {
        self.code.push(byte.into());
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        self.constants.len() as u8 - 1
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
