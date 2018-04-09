#[macro_use]
extern crate log;

pub mod dict;
pub mod ifo;
pub mod idx;
pub mod dictionary;
pub mod result;

use std::{fs, io, path};

pub struct StarDict {
    directories: Vec<dictionary::Dictionary>,
}

impl StarDict {
    pub fn new(root: path::PathBuf) -> result::Result<StarDict> {
        let mut items = Vec::new();
        if root.is_dir() {
            for it in try!(fs::read_dir(root)) {
                let it = try!(it).path();
                if !it.is_dir() {
                    return Err(result::Error::Io(io::Error::from(io::ErrorKind::NotFound)));
                }
                items.push(try!(dictionary::Dictionary::new(it)));
            }
        }

        return Ok(StarDict { directories: items });
    }

    pub fn info(&mut self) -> Vec<ifo::Ifo> {
        let mut items = Vec::new();
        for it in &mut self.directories {
            items.push(it.ifo.clone());
        }
        return items;
    }

    pub fn search(&mut self, word: &str) -> Vec<dictionary::Translation> {
        let mut items = Vec::new();
        for mut it in &mut self.directories {
            match it.search(word) {
                Ok(v) => items.push(dictionary::Translation {
                    info: it.ifo.clone(),
                    results: v,
                }),
                Err(e) => warn!("search {} in {} failed: {}", word, it.ifo.name, e),
            }
        }
        return items;
    }
}
