use super::Compiler;
use crate::lexer::token::tokentype::TokenType;
use super::precedence::Precedence;
use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Compiler)>,
    pub infix: Option<fn(&mut Compiler)>,
    pub precedence: Precedence,
}

/// == Precedence ==
/// None = 0,
/// Assignment, // =
/// Or,         // or
/// And,        // and
/// Equality,   // == !=
/// Comparison, // < > <= >=
/// Term,       // + -
/// Factor,     // * /
/// Unary,      // ! -
/// Call,       // . ()
/// Primary,    //


static RULES: OnceLock<[ParseRule; 256]> = OnceLock::new();

pub fn get_rule(token_type: TokenType) -> &'static ParseRule {
    RULES.get_or_init(|| {
        let mut rules: [ParseRule; 256] = [ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }; 256];

        rules[TokenType::LeftParen as usize] = ParseRule {
            prefix: Some(Compiler::grouping),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::RightParen as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::LeftBrace as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::RightBrace as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Comma as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Dot as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Minus as usize] = ParseRule {
            prefix: Some(Compiler::unary),
            infix: Some(Compiler::binary),
            precedence: Precedence::Term,
        };

        rules[TokenType::Plus as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Term,
        };

        rules[TokenType::Semicolon as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Slash as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Factor,
        };

        rules[TokenType::Star as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Factor,
        };

        rules[TokenType::Bang as usize] = ParseRule {
            prefix: Some(Compiler::unary),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::BangEqual as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Equality,
        };

        rules[TokenType::Equal as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::EqualEqual as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Equality,
        };

        rules[TokenType::Greater as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Comparison,
        };

        rules[TokenType::GreaterEqual as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Comparison,
        };

        rules[TokenType::Less as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Comparison,
        };

        rules[TokenType::LessEqual as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Comparison,
        };

        rules[TokenType::Identifier as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::String as usize] = ParseRule {
            prefix: Some(Compiler::string),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Number as usize] = ParseRule {
            prefix: Some(Compiler::number),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::And as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Else as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::False as usize] = ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::For as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::If as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Nil as usize] = ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Or as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Return as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::True as usize] = ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::While as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Let as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::SelfKw as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Form as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Fn as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Item as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Error as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Eof as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Dummy as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules
    }).get(token_type as usize).unwrap()
}

