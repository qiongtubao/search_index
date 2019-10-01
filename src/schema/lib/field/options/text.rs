use std::borrow::Cow;
use crate::schema::lib::field::options::index_record::IndexRecordOption;

#[derive(Debug)]
pub struct TextOptions {
    indexing: Option<TextFieldIndexing>,
    stored: bool, //是否保存
}
impl TextOptions {
    pub fn get_indexing_options(&self) -> Option<&TextFieldIndexing>{
        self.indexing.as_ref()
    }

}

#[derive(Debug)]
pub struct TextFieldIndexing {
    record: IndexRecordOption,
    tokenizer: Cow<'static, str>,
}
pub const STRING: TextOptions = TextOptions {
    indexing: Some(TextFieldIndexing {
        tokenizer: Cow::Borrowed("raw"),
        record: IndexRecordOption::Basic,
    }),
    stored: false,
};