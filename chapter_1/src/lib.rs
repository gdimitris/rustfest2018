#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    /// Should match a combination of letters a-z and A-Z.
    /// We don't need underscores but feel free to add them.
    Identifier,

    /// We will just need integers, so sequences of digits 0-9
    /// will suffice
    Number,

    /// '+'
    Add,

    /// '-'
    Subtract,

    /// '*'
    Multiply,

    /// '/'
    Divide,

    /// '='
    Assign,

    /// ';'
    Semicolon,
}

/// This struct needs some fields!
pub struct Lexer<'a> {
    src: &'a str
}


impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer { src: source }  
    }

    fn determine_token(b: u8) -> Option<Token> {
        match b as char {
            '+' => Some(Token::Add),
            '-' => Some(Token::Subtract),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '=' => Some(Token::Assign),
            ';' => Some(Token::Semicolon),
            '0'...'9' => Some(Token::Number),
            'a'...'z' => Some(Token::Identifier),
            'A'...'Z' => Some(Token::Identifier),
            _  => None
        }
    }
}

/// We will also use the `Iterator` trait from the
/// standard library for our Lexer.
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        None
    }

    fn eq(&mut self) -> Option<Token> {
        None
    }

}

#[cfg(test)]
#[test]
fn test() {
    let source = "four = 2 + 2; omg = 12345 / 0;";

    let expect = &[
        Token::Identifier,
        Token::Assign,
        Token::Number,
        Token::Add,
        Token::Number,
        Token::Semicolon,
        Token::Identifier,
        Token::Assign,
        Token::Number,
        Token::Divide,
        Token::Number,
        Token::Semicolon,
    ];

    // Create an iterator of Tokens out of the slice here.
    let expect = expect.iter().cloned();
    let lexer = Lexer::new(source);

    // We can use the `eq` method of the `Iterator` trait
    // to check that they are equal
    assert!(lexer.eq(expect));
}
