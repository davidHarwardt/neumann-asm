mod parser;
pub use parser::*;

#[cfg(test)]
mod tests {
    use crate::{parser::language::make_tokenizer, tokenizer::Tokenizer, microcode::keywords_from_microcode};

    #[test]
    fn test_main() {
        let lang_tok = make_tokenizer(keywords_from_microcode(include_str!("./lang/default.mc")));

        let tokens = lang_tok.digest_all(include_str!("./lang/tokens.asm"));

        match tokens {
            Ok(v) => println!("tokens: {:?}", &v),
            Err(err) => println!("error: {:?}", &err),
        }
    }
}
