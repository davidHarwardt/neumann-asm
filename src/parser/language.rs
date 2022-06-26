use crate::{multiple, string, tokenizer::{StringTokenizer, Tokenizer, Keyword}};
use super::tokenizer::{
    WhitespaceTokenizer,
    SymbolTokenizer, 
    MultipleTokenizer,
    StringLiteralTokenizer,
    NumberLiteralTokenizer,
    CommentTokenizer,
    TokenType,
    Char
};

pub fn make_tokenizer(instruction_tokenizers: Vec<StringTokenizer>) -> MultipleTokenizer {
    let instruction_tokenizers: Vec<Box<dyn Tokenizer>> = instruction_tokenizers.iter()
        .map(|v| Box::new(v.clone()) as Box<dyn Tokenizer>)
        .collect();


    multiple![
        multiple![
            string!(":", TokenType::Char(Char::Colon)),
            string!("%", TokenType::Char(Char::Percent)),
            string!("$", TokenType::Char(Char::Dollar)),
            string!(",", TokenType::Char(Char::Comma)),
            string!("!", TokenType::Char(Char::Bang))
        ],

        multiple![
            string!("macro",    TokenType::Keyword(Keyword::Macro)),
            string!("include",  TokenType::Keyword(Keyword::Include)),
            string!("define",   TokenType::Keyword(Keyword::Define)),
            string!("end",      TokenType::Keyword(Keyword::End))
        ],

        MultipleTokenizer { tokenizers: instruction_tokenizers },

        StringLiteralTokenizer {},
        NumberLiteralTokenizer {},

        SymbolTokenizer {},

        WhitespaceTokenizer {},
        CommentTokenizer {}
    ]
}
