use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;
use practice_language_compiler::lexer::{Token, TokenKind, Position};

const SRC: &str = r#"  "Hello World" "#;

fn main() {
    let id_rules = Regex::new("[_a-zA-Z][_a-zA-Z0-9]*").unwrap();
    println!("? {}", id_rules.is_match(" "));

    let opers = ["+=", "==", "=", "+", "-", "*", "/", "(", ")", ";", "#"];

    let graphemes: Vec<&str> = SRC.graphemes(true).collect();
    let len = SRC.len();
    
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
            buffer = &SRC[buffer_start..=ptr];
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
    
    println!("Tokens: {:#?}", &tokens);
    println!("SRC:\n{}", &SRC);
}