use unicode_segmentation::UnicodeSegmentation; // 1.10.1
use regex::Regex; // 1.7.1

#[derive(Debug)]
pub struct Position {
  pub line: u32,
  pub col: u32
}

#[derive(Debug)]
pub enum TokenKind {
  Identifier(&'static str),
  Operator(&'static str),
  IntegerLiteral(i32),
  FloatLiteral(f64),
  StringLiteral(&'static str),
  EOF
}

#[derive(Debug)]

pub struct Token {
  pub kind: TokenKind,
  pub pos: Option<Position>
}

impl Token {
  pub const EOF: Token = Token { kind: TokenKind::EOF, pos: None };
}

