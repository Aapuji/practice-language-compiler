use unicode_segmentation::UnicodeSegmentation; // 1.10.1
use regex::{Regex, RegexBuilder}; // 1.7.1

#[derive(Debug)]
pub struct Position {
  pub line: u32,
  pub col: u32
}

#[derive(Debug)]
pub enum TokenKind {
  Identifier(&'static str),
  Keyword(&'static str),
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
  let ops = ["+=", "==", "=", "+", "-", "*", "/", "(", ")", ";", "#"];
  let keywords = ["print"];

  let graphemes: Vec<&str> = src.graphemes(true).collect();
  let len = src.len();
  
  let mut tokens: Vec<Token> = Vec::new();
  let mut ptr: usize = 0;
  let mut buffer_start: usize = 0;
  let mut buffer: &'static str = "";
  let mut stack: Vec<&str> = Vec::new();

  let mut line = 1;
  let mut col = 1;
  let mut line_start = 1;
  let mut col_start = 1;

  // Finds ident, keyword, or operator from word (ie. buffer)
  let token_from_word = |word: &'static str, line: u32, col: u32| -> Option<Token> {
    if let Some(&val) = ops.iter().find(|&e| &word == e) {
      return Some(Token {
        kind: TokenKind::Operator(val),
        pos: Some(Position { line, col })
      });
    }

    if id_rules.is_match(&word) {
      return Some(Token {
        kind: if let Some(&val) = keywords.iter().find(|&e| &word == e) {
          TokenKind::Keyword(val)
        } else {
          TokenKind::Identifier(word)
        },
        pos: Some(Position { line, col })
      });
    }

    None
  };

  let slice_next = |start: usize, end: usize| &src[start..=end];

  while ptr < len {
    println!("${}, {}: {}: {}$", ptr, buffer_start, graphemes[ptr], &buffer);

    let substr: &str = graphemes[ptr];
    let last = stack.last();

    if buffer.is_empty() {
      buffer_start = ptr;
      line_start = line;
      col_start = col;
    }

    if substr == "\n" { // Check newline
      line += 1;
      col = 1;
      
      match last {
        Some(&"\"") => buffer = slice_next(buffer_start, ptr), // Check if in string. If so, add to buffer
        Some(&"#") => { stack.pop(); () }, // Check if in single-line comment. If so, remove comment.
        _ => todo!()
      }
    } else if let Some(&"#") = last { // Check if text is in single-line comment.
      ()
    } else if let Some(&"\"") = last { // Check if text is currently in string. 
      if substr == "\"" {
        tokens.push(Token {
          kind: TokenKind::StringLiteral(buffer),
          pos: Some(Position { line, col })
        });
        stack.pop();
        buffer = "";
      } else {
        buffer = slice_next(buffer_start, ptr);
      }
    } else if substr == "#" {
      stack.push("#");
    } else if substr == "\"" { // Check if the character is a " and start string literal.
      stack.push("\"");
      token_from_word(buffer, line_start, col_start);
    } else if id_rules.is_match(substr) { // Find identifiers.
      buffer = slice_next(buffer_start, ptr);
    } else {
      if !buffer.is_empty() {
        let token = token_from_word(buffer, line_start, col_start);
    
        match token {
          Some(tok) => tokens.push(tok),
          None => ()
        }
      }
    }

    ptr += 1;
    col += 1;
  }

  if !buffer.is_empty() {
    let token = token_from_word(buffer, line_start, col_start);

    match token {
      Some(tok) => tokens.push(tok),
      None => ()
    }
  }

  tokens.push(Token::EOF);

  tokens
}
