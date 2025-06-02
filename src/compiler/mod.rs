pub mod rules;
pub mod precedence;

use crate::lexer::{Lexer, token::{Token, tokentype::TokenType}};
use crate::chunk::{Chunk, Values, Obj, ObjType, ObjString};
use crate::hash;
use rules::get_rule;
use precedence::Precedence;
use crate::opcode::*;
use std::alloc::{self, Layout};

pub struct Compiler {
    token_stream: Lexer,
    chunk: Chunk,
    current: Token,
    previous: Token,
    is_error: bool,
}

impl Compiler {
    pub fn new(token_stream: Lexer) -> Self {
        Self {
            token_stream,
            chunk: Chunk::new(2048),
            current: Token::dummy(),
            previous: Token::dummy(),
            is_error: false,
        }
    }

    pub fn compile(mut self) -> Option<Chunk> {
        self.advance();
        self.parse_precedence(Precedence::Assignment);
        self.output_chunk()
    }

    pub fn output_chunk(mut self) -> Option<Chunk> {
        if self.is_error {
            return None;
        }
        self.chunk.write_byte(OP_RETURN, self.previous.line as u32);
        
        #[cfg(debug_assertions)]
        {
            self.chunk.chunk_peek("test at Compiler");
        }

        Some(self.chunk)
    }

    pub fn consume(&mut self, expected: TokenType) {
        if self.current.token_type == expected { return self.advance(); }
        self.error_at_current();
    }

    pub fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.token_stream.scan_token();
            if self.current.token_type != TokenType::Error { break; }

            self.error_at_current();
        }
    }

    pub fn error_at_current(&mut self) {
        let token = self.current;
        
        let error_message_or_error_token = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(token.start, token.length))
        };
        eprintln!("[line {}] Error! : 'error occured at \"{}\"...'", token.line, error_message_or_error_token);

        self.is_error = true;
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        if let Some(prefix_fn) = get_rule(self.previous.token_type).prefix {
            prefix_fn(self);
        } else {
            self.error_at_current();
            return;
        }

        while precedence <= get_rule(self.current.token_type).precedence {
            self.advance();

            if let Some(infix_fn) = get_rule(self.previous.token_type).infix {
                infix_fn(self);
            } else {
                self.error_at_current();
                break;
            }
        }
    }

    pub fn number(&mut self) {
        let token = self.previous;
        let lexeme = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(token.start, token.length))
        };
        let value = lexeme.parse::<f64>().expect("Invalid number literal");
        self.chunk.write_constant(Values::Number(value), token.line as u32);
    }

    pub fn grouping(&mut self) {
        self.parse_precedence(Precedence::Assignment);
        self.consume(TokenType::RightParen);
    }

    pub fn unary(&mut self) {
        let operator = self.previous;
        let line = operator.line as u32;
        self.parse_precedence(Precedence::Unary);

        match operator.token_type {
            TokenType::Minus | TokenType::Bang => self.chunk.write_byte(OP_NEGATE, line),
            _ => self.error_at_current(),
        }
    }

    pub fn binary(&mut self) {
        let operator = self.previous;
        let line = operator.line as u32;
        let rule = get_rule(operator.token_type);
        self.parse_precedence(rule.precedence.next());

        match operator.token_type {
            TokenType::Plus => self.chunk.write_byte(OP_ADD, line),
            TokenType::Minus => self.chunk.write_byte(OP_SUBTRACT, line),
            TokenType::Star => self.chunk.write_byte(OP_MULTIPLY, line),
            TokenType::Slash => self.chunk.write_byte(OP_DIVIDE, line),
            TokenType::EqualEqual => self.chunk.write_byte(OP_EQUAL, line),
            TokenType::Greater => self.chunk.write_byte(OP_GREATER, line),
            TokenType::Less => self.chunk.write_byte(OP_LESS, line),
            TokenType::BangEqual => {
                self.chunk.write_byte(OP_EQUAL, line);
                self.chunk.write_byte(OP_NEGATE, line);
            },
            TokenType::GreaterEqual => {
                self.chunk.write_byte(OP_LESS, line);
                self.chunk.write_byte(OP_NEGATE, line);
            },
            TokenType::LessEqual => {
                self.chunk.write_byte(OP_GREATER, line);
                self.chunk.write_byte(OP_NEGATE, line);
            }
            _ => self.error_at_current(),
        }
    }

    pub fn literal(&mut self) {
        let literal = self.previous;
        let line = literal.line as u32;

        match literal.token_type {
            TokenType::True => self.chunk.write_byte(OP_TRUE, line),
            TokenType::False => self.chunk.write_byte(OP_FALSE, line),
            TokenType::Nil => self.chunk.write_byte(OP_NIL, line),
            _ => self.error_at_current(),
        }
    }

    pub fn string(&mut self) {
        let token = self.previous;
        let obj_ptr: *mut Obj = unsafe { make_obj_str(token.start.add(1), token.length - 2) } as *mut Obj;
        self.chunk.write_constant(Values::Obj(obj_ptr), token.line as u32);
    }
}

pub unsafe fn make_obj_str(start: *const u8, length: usize) -> *mut ObjString {
    let str_layout = Layout::array::<u8>(length + 1).unwrap();
    let chars_ptr: *mut u8 = unsafe { alloc::alloc(str_layout) };
   
    unsafe {
        std::ptr::copy_nonoverlapping(start, chars_ptr, length);
    }

    let obj_str_layout = Layout::new::<ObjString>();
    let obj_ptr = unsafe { alloc::alloc(obj_str_layout) as *mut ObjString };
    let hash = hash::fnv1a_hash(chars_ptr, length);

    unsafe {
        std::ptr::write(obj_ptr, ObjString {
            obj: Obj { type_obj: ObjType::String, next: std::ptr::null_mut(), },
            length,
            chars: chars_ptr,
            hash,
        });
    }

    obj_ptr
}

