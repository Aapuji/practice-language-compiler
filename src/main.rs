use practice_language_compiler::lexer;

const SRC: &str = r#"  "Hello World" "#;

fn main() {
    let tokens = lexer::lex(&SRC);

    println!("Tokens: {:#?}", &tokens);
    println!("SRC:\n{}", &SRC);
}