pub mod token;
pub use token::*;
pub use token::TokenType;

pub struct Lexer {
    start: *const u8,
    current: *const u8,
    line: usize,
}

impl Lexer {
    pub fn new(tokens: &[u8]) -> Self {
        let ptr = tokens.as_ptr();
        Self {
            start: ptr,
            current: ptr,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        
        if self.is_end() { return self.make_token(TokenType::Eof); }
        
        if self.is_alpha() {
            self.advance();
            while self.is_alpha() || self.is_digit(0) { self.advance(); }
            return self.make_token(self.identifier());
        }

        if self.is_digit(0) {
            self.advance();
            while self.is_digit(0) { self.advance(); }
            if self.peek() == b'.' && self.is_digit(1) {
                self.advance();
                while self.is_digit(0) { self.advance(); }
            }
            return self.make_token(TokenType::Number);
        }

        match self.advance() {
            b'(' => return self.make_token(TokenType::LeftParen),
            b')' => return self.make_token(TokenType::RightParen),
            b'{' => return self.make_token(TokenType::LeftBrace),
            b'}' => return self.make_token(TokenType::RightBrace),
            b',' => return self.make_token(TokenType::Comma),
            b'.' => return self.make_token(TokenType::Dot),
            b'-' => return self.make_token(TokenType::Minus),
            b'+' => return self.make_token(TokenType::Plus),
            b';' => return self.make_token(TokenType::Semicolon),
            b'/' => return self.make_token(TokenType::Slash),
            b'*' => return self.make_token(TokenType::Star),
            b'!' => {
                if self.match_byte(b'=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            },
            b'=' => {
                if self.match_byte(b'=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    return self.make_token(TokenType::Equal);
                }
            },
            b'>' => {
                if self.match_byte(b'=') {
                    return self.make_token(TokenType::GreaterEqual);
                } else {
                    return self.make_token(TokenType::Greater);
                }
            },
            b'<' => {
                if self.match_byte(b'=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    return self.make_token(TokenType::Less);
                }
            },
            b'"' => return self.make_string_token(), 
            _ => {},
        }

        Token::error("Unexpected character!", self.line)
    }

    pub fn is_end(&self) -> bool {
        unsafe { *self.current == (b'\0' as u8) }
    }

    pub fn is_digit(&self, idx: usize) -> bool {
        unsafe {
            let byte = *self.current.add(idx);
            byte >= b'0' && byte <= b'9'
        }
    }

    pub fn is_alpha(&self) -> bool {
        unsafe {
            let byte = *self.current;
            (byte >= b'a' && byte <= b'z') ||
            (byte >= b'A' && byte <= b'Z') ||
            byte == b'_'
        }
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.start, self.current, self.line)
    }

    pub fn advance(&mut self) -> u8 {
        let byte: u8 = unsafe { *self.current };
        self.current = unsafe { self.current.add(1) };
        byte
    }

    pub fn match_byte(&mut self, expected: u8) -> bool {
        if self.is_end() { return false; }
        let byte = unsafe { *self.current };
        if byte != expected { return false; }
        self.current = unsafe { self.current.add(1) };
        true
    }

    #[inline(always)]
    pub fn skip_whitespace(&mut self) {
        while matches!(self.peek(), b' ' | b'\r' | b'\t' | b'\n' | b'/') {
            let mut byte = unsafe { *self.current };
            if byte == b'\n' {
                self.line += 1;
            } else if self.peek_next() == b'/' {
                while (byte != b'\n') && !self.is_end() {
                    self.advance();
                    byte = unsafe { *self.current };
                }
                self.line += 1;
            }
            self.advance();
        }

    }

    pub fn make_string_token(&mut self) -> Token {
        while !self.is_end() {
            let byte = unsafe { *self.current };
            if byte == b'"' { break; }
            if byte == b'\n' { self.line += 1; }
            self.advance();
        }

        if self.is_end() {
            return Token::error("Unterminated String", self.line);
        }
        self.advance();

        self.make_token(TokenType::String)
    }

    pub fn peek_next(&self) -> u8 {
        if self.is_end() {
            0
        } else {
            unsafe { *self.current.add(1) }
        }
    }

    #[inline(always)]
    pub fn peek(&self) -> u8 {
        unsafe { *self.current }
    }

    pub fn identifier(&self) -> TokenType {
        let len = self.current as usize - self.start as usize;
        let start = self.start;

        let first = unsafe { *start };
        match first {
            b'a' => self.check_keyword(start, len, b"and", TokenType::And),
            b'e' => self.check_keyword(start, len, b"else", TokenType::Else),
            b'i' => self.check_keyword(start, len, b"if", TokenType::If),
            b'l' => self.check_keyword(start, len, b"let", TokenType::Let),
            b'n' => self.check_keyword(start, len, b"nil", TokenType::Nil),
            b'o' => self.check_keyword(start, len, b"or", TokenType::Or),
            b'r' => self.check_keyword(start, len, b"return", TokenType::Return),
            b's' => self.check_keyword(start, len, b"self", TokenType::SelfKw),
            b't' => self.check_keyword(start, len, b"true", TokenType::True),
            b'w' => self.check_keyword(start, len, b"while", TokenType::While),
            b'I' => self.check_keyword(start, len, b"Item", TokenType::Item),
            b'f' => {
                if len >= 2 {
                    let second = unsafe { *start.add(1) };
                    match second {
                        b'a' => self.check_keyword(start, len, b"false", TokenType::False),
                        b'o' => self.check_keyword(start, len, b"for", TokenType::For),
                        _ => TokenType::Identifier,
                    }
                } else { TokenType::Identifier }
            },
            b'F' => {
                if len >= 2 {
                    let second = unsafe { *start.add(1) };
                    match second {
                        b'n' => self.check_keyword(start, len, b"Fn", TokenType::Fn),
                        b'o' => self.check_keyword(start, len, b"Form", TokenType::Form),
                        _ => TokenType::Identifier,
                    }
                } else { TokenType::Identifier }
            },
            _ => TokenType::Identifier,
        }
    }
    
    #[inline(always)]
    fn check_keyword(
        &self,
        start: *const u8,
        len: usize,
        expected: &[u8],
        tokentype: TokenType,
    ) -> TokenType {
        if len != expected.len() { return TokenType::Identifier }

        let slice = unsafe { std::slice::from_raw_parts(start, len) };
        if slice == expected {
            return tokentype;
        } else {
            return TokenType::Identifier;
        }
    }
}

