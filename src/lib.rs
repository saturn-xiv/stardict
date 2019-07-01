#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

pub mod dict;
pub mod dictionary;
pub mod idx;
pub mod ifo;
pub mod result;

use std::{fs, path};

pub struct StarDict {
    directories: Vec<dictionary::Dictionary>,
}

impl StarDict {
    pub fn new(root: path::PathBuf) -> result::Result<StarDict> {
        let mut items = Vec::new();
        if root.is_dir() {
            for it in fs::read_dir(root)? {
                let it = it?.path();
                if it.is_dir() {
                    match dictionary::Dictionary::new(it) {
                        Ok(it) => {
                            items.push(it);
                        }
                        Err(e) => {
                            error!("ignore reason: {}", e);
                        }
                    }
                }
            }
        }

        Ok(StarDict { directories: items })
    }

    pub fn info(&mut self) -> Vec<ifo::Ifo> {
        let mut items = Vec::new();
        for it in &mut self.directories {
            items.push(it.ifo.clone());
        }
        items
    }

    pub fn search(&mut self, word: &str) -> Vec<dictionary::Translation> {
        let mut items = Vec::new();
        for it in &mut self.directories {
            match it.search(word) {
                Ok(v) => items.push(dictionary::Translation {
                    info: it.ifo.clone(),
                    results: v,
                }),
                Err(e) => warn!("search {} in {} failed: {}", word, it.ifo.name, e),
            }
        }
        items
    }
}
