
#[derive(Debug)]
pub struct Token {
    pub from: usize,
    pub to: usize,
    pub token_type: TokenType,
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Macro,
    Include,
    Define,
    End,
    Instruction(u64, String),
}

#[derive(Debug, Clone)]
pub enum Char {
    Semicolon,
    Colon,
    Percent,
    Dollar,
    Comma,
    Bang,
}

#[derive(Debug, Clone)]
pub enum BracketType {
    Opening,
    Closing,
}

#[derive(Debug, Clone)]
pub enum Brace {
    Curly(BracketType),
    Square(BracketType),
    Round(BracketType),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Whitespace,
    StringLiteral(String),
    NumberLiteral(i64),

    // string - special chars
    Char(Char),
    // end

    Brace(Brace),
    Operator(Operator),

    // string - keywords
    Keyword(Keyword),
    // end

    Symbol(String),
}

pub struct DigestResult {
    length: usize,
    token: TokenType,
}


#[derive(Debug)]
pub enum TokenizerError {
    UnableToParseToken(FilePosition),
}

#[derive(Debug)]
pub struct FilePosition {
    pub line: usize,
    pub character: usize
}

impl FilePosition {
    pub fn from_index(file: &str, idx: usize) -> Self {
        let mut line = 1;
        let mut character = 0;
    
        for c in file.chars().take(idx) {
            character += 1;
            if c == '\n' {
                character = 0;
                line += 1;
            }
        }
    
        Self { line, character }
    }
}


pub trait Tokenizer {
    fn digest(&self, string: &str) -> Option<DigestResult>;

    fn digest_all(&self, string: &str) -> Result<Vec<Token>, TokenizerError> {
        let mut current_str = &string[..];
        let mut tokens = Vec::new();
        let mut idx = 0;

        while current_str.len() > 0 {
            match self.digest(current_str) {
                Some(tok) => {
                    let old_idx = idx;
                    idx += tok.length;
                    current_str = &string[idx..];
                    tokens.push(Token {
                        token_type: tok.token,
                        from: old_idx,
                        to: idx,
                    });
                },
                None => {
                    // error
                    return Err(TokenizerError::UnableToParseToken(FilePosition::from_index(&string, idx)));
                },
            }
        }

        Ok(tokens)
    }
}

pub struct MultipleTokenizer {
    pub tokenizers: Vec<Box<dyn Tokenizer>>
}

impl Tokenizer for MultipleTokenizer {
    fn digest(&self, string: &str) -> Option<DigestResult> {
        for tok in &self.tokenizers {
            match tok.digest(string) {
                Some(v) => return Some(v),
                None => {},
            }
        }
        None
    }
}

pub struct StringLiteralTokenizer {}
impl Tokenizer for StringLiteralTokenizer {
    fn digest(&self, string: &str) -> Option<DigestResult> {
        let mut iter = string.chars();

        match iter.next() {
            Some(v) if v == '"' => {
                let res: String = iter
                    .take_while(|c| *c != '"')
                    .collect();

                Some(DigestResult {
                    length: res.len() + 2,
                    token: TokenType::StringLiteral(res)
                })
            },
            _ => None,
        }
    }
}

pub struct NumberLiteralTokenizer {}
impl Tokenizer for NumberLiteralTokenizer {
    fn digest(&self, string: &str) -> Option<DigestResult> {
        // todo add 0x syntax
        match string.chars().next() {
            Some(v) if v.is_numeric() || v == '-' => {
                let number_str: String = string.chars().take_while(|c| c.is_numeric()).collect();
                match number_str.parse::<i64>() {
                    Ok(v) => Some(DigestResult {
                        length: number_str.len(),
                        token: TokenType::NumberLiteral(v),
                    }),
                    Err(_) => None,
                }
            },
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StringTokenizer {
    pub string: String,
    pub token: TokenType,
}

impl Tokenizer for StringTokenizer {
    fn digest(&self, string: &str) -> Option<DigestResult> {
        if string.starts_with(&self.string) {
            Some(DigestResult {
                length: self.string.len(),
                token: self.token.clone(),
            })
        }   
        else { None }
    }
}

pub struct SymbolTokenizer {}
impl Tokenizer for SymbolTokenizer {
    fn digest(&self, string: &str) -> Option<DigestResult> {
        let mut iter = string.chars();
        match iter.next() {
            Some(v) if v.is_alphabetic() || v == '_' => {
                let mut res: String = v.to_string();
                let iter_res: String = iter
                    .take_while(|c| c.is_alphabetic() || *c == '_' || c.is_numeric())
                    .collect();
                res.push_str(&iter_res);

                Some(DigestResult {
                    length: res.len(),
                    token: TokenType::Symbol(res)
                })
            },
            _ => None,
        }
    }
}

pub struct WhitespaceTokenizer {}
impl Tokenizer for WhitespaceTokenizer {
    fn digest(&self, string: &str) -> Option<DigestResult> {
        let res: String = string.chars()
            .take_while(|c| c.is_whitespace())
            .collect();

        let length = res.len();
        if length > 0 { Some(DigestResult { length, token: TokenType::Whitespace }) }
        else { None }
    }
}

pub struct CommentTokenizer {}
impl Tokenizer for CommentTokenizer {
    fn digest(&self, string: &str) -> Option<DigestResult> {
        if string.starts_with(";") {
            let length = string.chars()
                .take_while(|c| *c != '\n')
                .count();

            Some(DigestResult {
                length: length + 1,
                token: TokenType::Whitespace,
            })
        }
        else { None }
    }
}

#[macro_export]
macro_rules! multiple {
    ($($tok:expr),+) => {
        crate::parser::tokenizer::MultipleTokenizer { tokenizers: vec![$(Box::new($tok)),+], }
    };
}

#[macro_export]
macro_rules! string {
    ($name:literal, $token:expr) => {
        crate::parser::tokenizer::StringTokenizer { string: $name.to_owned(), token: $token }
    };
}

// whitespace-tokenizer         tokenizes whitespace
// string-literal-tokenizer     tokenizes string literals ("asdf")
// number-literal-tokenizer     tokenizes number literals (123, 0xff, ...)
// string-tokenizer             tokenizes strings of chars: matches token
// symbol-tokenizer             tokenizes symbols ([letter|_][letter|number|_]*)

// multiple-tokenizer           combines multiple tokenizers


