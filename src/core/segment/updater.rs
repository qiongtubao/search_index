use crate::schema::lib::Schema;
use crate::directory::lib::Directory;
use crate::Result;
use crate::directory::META_FILEPATH;
use crate::core::index::meta::IndexMeta;
use std::io::Write;
fn save_metas(metas: &IndexMeta, directory: &mut dyn Directory) -> Result<()> {
    let mut buffer = serde_json::to_vec_pretty(metas)?;
    writeln!(&mut buffer)?;
    directory.atomic_write(&META_FILEPATH, &buffer[..])?;
    Ok(())
}

pub fn save_new_metas(schema: Schema, directory: &mut dyn Directory) -> Result<()> {
    save_metas(
        &IndexMeta {
            segments: Vec::new(),
            schema: schema,
            opstamp: 0u64,
            payload: None,
        },
        directory,
    )
}
