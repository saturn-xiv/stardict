use std::path;

use super::result::Result;

pub struct Dict {}

impl Dict {
    pub fn open(_file: path::PathBuf) -> Result<Dict> {
        return Ok(Dict {});
    }
}
