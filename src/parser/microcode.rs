
use crate::tokenizer::{StringTokenizer, TokenType, Keyword};

pub fn keywords_from_microcode(micro_code: &str) -> Vec<StringTokenizer> {
    let iter = micro_code.lines()
        .skip_while(|l| if let Some(c) = l.chars().nth(0) { c.is_numeric() } else { false });

    let mut tokenizers = Vec::new();
    for (i, line) in iter.enumerate() {
        if !line.is_empty() {
            let line = line.to_lowercase();
            tokenizers.push(StringTokenizer {
                string: line.to_owned(),
                token: TokenType::Keyword(Keyword::Instruction(i.try_into().unwrap_or(0) * 10, line.to_owned()))
            })
        }
    }
    tokenizers
}
