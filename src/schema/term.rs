use crate::schema::field::Field;
use std::fmt::Write;
#[derive(Debug)]
pub struct Term<'a> {
    pub field: Field,
    pub text: &'a str,
}

impl<'a> Term<'a> {
    pub fn write_into(&self, term_str: &mut String) {
        term_str.clear();
        let Field(field_idx) = self.field;
        term_str.write_fmt(format_args!("{}:{}", field_idx, self.text));
    }
}