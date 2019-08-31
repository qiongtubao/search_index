
use regex::Regex;

lazy_static! {
    static ref WORD_PTN: Regex = Regex::new(r"[a-zA-Z0-9]+").unwrap();
}
/**
    分词
*/
pub struct TokenIter<'a> {
    text: &'a str,
    token_it: Box<Iterator<Item=(usize, usize)> + 'a>
}
impl<'a> Iterator for TokenIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        self.token_it.next().map(|(start, end)|{
              &self.text[start..end]
        })
    }
}
pub fn tokenize<'a>(text: &'a str) -> TokenIter {
    TokenIter {
        text,
        token_it: Box::new(WORD_PTN.find_iter(text)),
    }
}

#[cfg(test)]
mod analyzer {
    use crate::analyzer::tokenize;

    #[test]
    fn tokenizer() {
       let words: Vec<&str> = tokenize("hello happy tax payer!").collect();
        assert_eq!(words, vec!("hello", "happy", "tax", "payer"));
    }
}