pub mod lib;
pub mod document;
#[cfg(test)]
mod tests {
    use crate::schema::lib::field::options::text::STRING;
    use crate::schema::lib::Schema;

    pub fn build() {
        let mut schema_builder = Schema::builder();
        let field_str = schema_builder.add_text_field("field_str", STRING);
        let schema = schema_builder.build();
    }
}