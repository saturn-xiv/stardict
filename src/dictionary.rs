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

    pub fn search(&mut self, word: &str) -> Result<Vec<TranslationItem>> {
        match self.ifo.version.as_ref() {
            "2.4.2" => self.search242(word),
            "3.0.0" => self.search300(word),
            _ => Err(Error::Io(io::Error::from(io::ErrorKind::InvalidData))),
        }
    }

    fn search242(&mut self, _word: &str) -> Result<Vec<TranslationItem>> {
        let mut items = Vec::new();
        items.push(TranslationItem {
            mode: 'h',
            body: "hi v2.4.2".to_string(),
        });
        return Ok(items);
    }

    fn search300(&mut self, _word: &str) -> Result<Vec<TranslationItem>> {
        let mut items = Vec::new();
        items.push(TranslationItem {
            mode: 'h',
            body: "hi v3.0.0".to_string(),
        });
        return Ok(items);
    }
}

#[derive(Debug, Clone)]
pub struct Translation {
    pub info: Ifo,
    pub results: Vec<TranslationItem>,
}

#[derive(Debug, Clone)]
pub struct TranslationItem {
    pub mode: char,
    pub body: String,
}
