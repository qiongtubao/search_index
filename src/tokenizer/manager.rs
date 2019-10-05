use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use crate::tokenizer::lib::boxed::BoxedTokenizer;
use crate::tokenizer::raw::RawTokenizer;
use crate::tokenizer::simple::SimpleTokenizer;
use crate::tokenizer::lib::Tokenizer;
use crate::tokenizer::filter::lower_caser::LowerCaser;
use crate::tokenizer::filter::remove_long::RemoveLongFilter;

#[derive(Clone)]
pub struct TokenizerManager {
    tokenizers: Arc<RwLock<HashMap<String, BoxedTokenizer>>>,
}

impl TokenizerManager {
    pub fn register<A>(&self, tokenizer_name: &str, tokenizer: A)
        where
            A: Into<BoxedTokenizer>,
    {
        let boxed_tokenizer = tokenizer.into();
        self.tokenizers
            .write()
            .expect("Acquiring the lock should never fail")
            .insert(tokenizer_name.to_string(), boxed_tokenizer);
    }
}

impl Default for TokenizerManager {
    fn default() -> TokenizerManager {
        let manager = TokenizerManager {
            tokenizers: Arc::new(RwLock::new(HashMap::new())),
        };
        //这里还没注册 raw,default,en_stem  Tokenizer
        manager.register("raw", RawTokenizer);
        manager.register(
            "default",
            SimpleTokenizer
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser),
        );
//        manager.register(
//            "en_stem",
//            SimpleTokenizer
//                .filter(RemoveLongFilter::limit(40))
//                .filter(LowerCaser)
//                .filter(Stemmer::new(Language::English)),
//        );
        manager
    }

}