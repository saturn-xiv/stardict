use std::{io, path};

use super::result::{Error, Result};
use super::idx::Idx;
use super::ifo::Ifo;
use super::dict::Dict;

pub struct Dictionary {
    pub idx: Idx,
    pub ifo: Ifo,
    pub dict: Dict,
}

impl Dictionary {
    pub fn new(root: path::PathBuf) -> Result<Dictionary> {
        if let Some(name) = root.file_name() {
            if let Some(name) = name.to_str() {
                return Ok(Dictionary {
                    idx: try!(Idx::open(root.join(format!("{}.idx", name)))),
                    ifo: try!(Ifo::open(root.join(format!("{}.ifo", name)))),
                    dict: try!(Dict::open(root.join(format!("{}.dict", name)))),
                });
            }
        }
        return Err(Error::Io(io::Error::from(io::ErrorKind::NotFound)));
    }
}
