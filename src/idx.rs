use std::path;

use super::result::Result;

pub struct Idx {}

impl Idx {
    pub fn open(_file: path::PathBuf) -> Result<Idx> {
        return Ok(Idx {});
    }
}
