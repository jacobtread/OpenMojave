use std::str::Utf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EspError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    BinaryParseError(#[from] binrw::Error),

    #[error(transparent)]
    Utf8ParseError(#[from] Utf8Error),

    #[error("ISO-8859-1 Parse Error")]
    ISO88591ParseError(u32),

    #[error("String EOF")]
    StringEOF,

    #[error("Extra bytes after parsing record ({:?})", _0)]
    ExtraBytes(Vec<u8>),

    #[error("Duplicate field encountered: ({})", _0)]
    DuplicateField(String),

    #[error("Unknown {} record version: {}", _0, _1)]
    UnknownVersion(String, u16),

    #[error("Duplicate String ID encountered: ({})", _0)]
    DuplicateStringID(u32),
}
