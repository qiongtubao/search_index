extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg_attr(test, macro_use)]
extern crate serde_json;
mod directory;
mod schema;
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::directory::mmap::MmapDirectory;

    #[test]
    fn mmap() {
        let path = PathBuf::from(format!("tests/index"));
        if !path.is_dir() {
            std::fs::create_dir_all(&path).expect("ok?");
        }
        let dir = MmapDirectory::open(path).expect("mmapDirectory");

    }
}