pub mod tokentype;
pub use tokentype::*;

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: *const u8,
    pub length: usize,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start: *const u8, current: *const u8, line: usize) -> Self {
        Self {
            token_type,
            start,
            length: current as usize - start as usize,
            line,
        }
    }

    pub fn error(message: &str, line: usize) -> Self {
        Self {
            token_type: TokenType::Error,
            start: message.as_ptr(),
            length: message.len(),
            line,
        }
    }

    pub fn dummy() -> Self {
        Self {
            token_type: TokenType::Dummy,
            start: std::ptr::null(),
            length: 0,
            line: 0,
        }
    }
}

