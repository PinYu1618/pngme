//pub mod args;
pub mod chunk;
pub mod chunk_type;
//pub mod commands;
pub mod png;

#[derive(Debug)]
pub enum Error {
    InvalidChunkType,
    ChunkDataTooLarge,
    FromUtf8Error,
}

pub type Result<T> = std::result::Result<T, Error>;
