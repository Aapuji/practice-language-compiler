use std::borrow::Borrow;

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

pub fn lex(src: &'static str) -> Vec<Token> {
  let id_rules = Regex::new("[_a-zA-Z][_a-zA-Z0-9]*").unwrap();
  println!("? {}", id_rules.is_match(" "));

  let opers = ["+=", "==", "=", "+", "-", "*", "/", "(", ")", ";", "#"];

  let graphemes: Vec<&str> = src.graphemes(true).collect();
  let len = src.len();
  
  let mut tokens: Vec<Token> = Vec::new();
  let mut ptr: usize = 0;
  let mut buffer_start: usize = 0;
  let mut buffer: &str = "";
  let mut stack: Vec<&str> = Vec::new();

  let mut line = 1;
  let mut col = 0;

  while ptr < len {
    println!("${}, {}: {}: {}$", ptr, buffer_start, graphemes[ptr], &buffer);

    let substr: &str = graphemes[ptr];

    if buffer.is_empty() {
      buffer_start = ptr;
    }

    if substr == "\n" {
      line += 1;
      col = 0;
    } else if id_rules.is_match(substr) {
      buffer = &src[buffer_start..=ptr];
    } else if substr == "\"" {
      match stack.last() {
        Some(&"\"") => {
          tokens.push(Token {
            kind: TokenKind::StringLiteral(buffer),
            pos: Some(Position { line, col })
          });
          stack.pop();
          buffer = "";
        },
        _ => stack.push("\"")
      }
    }

    ptr += 1;
  }

  tokens.push(Token::EOF);

  tokens
}