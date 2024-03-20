use serde::{Deserialize, Serialize};

use crate::bytecode::Op;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    Integer(u64),
    Float(f64),
    String(String),
}

#[derive(Default, Serialize, Deserialize)]
pub struct Chunk {
    pub constants: Vec<Constant>,
    pub code: Vec<u8>,
}

impl Chunk {
    pub fn push(&mut self, op: Op) {
        self.code.extend(op.to_bytes())
    }

    pub fn add_constant(&mut self, constant: Constant) -> u8 {
        let idx = self.constants.len();
        assert!(idx < u8::MAX.into());

        self.constants.push(constant);
        idx as u8
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChunkReadError {
    #[error(transparent)]
    Deserialize(postcard::Error),

    #[error("extra bytes at end of file: {0:?}")]
    ExtraBytes(Vec<u8>),
}

#[derive(thiserror::Error, Debug)]
pub enum ChunkWriteError {
    #[error(transparent)]
    Serialize(postcard::Error),
}

impl Chunk {
    pub fn read(r: impl std::io::Read) -> Result<Self, ChunkReadError> {
        let mut extra_bytes = Vec::new();

        let (chunk, (_, _)) =
            postcard::from_io((r, &mut extra_bytes)).map_err(ChunkReadError::Deserialize)?;

        if extra_bytes.is_empty() {
            Ok(chunk)
        } else {
            Err(ChunkReadError::ExtraBytes(extra_bytes))
        }
    }

    pub fn write(&self, w: impl std::io::Write) -> Result<(), ChunkWriteError> {
        postcard::to_io(self, w).map_err(ChunkWriteError::Serialize)?;
        Ok(())
    }
}
