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
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    use std::path::Path;
    use super::*;

    fn fail(e: result::Error) {
        println!("fail: {:?}", e);
        assert!(false);
    }

    #[test]
    fn it_works() {
        env_logger::init();
        match StarDict::new(Path::new("/usr").join("share").join("stardict").join("dic")) {
            Ok(mut st) => for it in st.info() {
                println!("{:?}", it);
            },
            Err(e) => fail(e),
        }
    }
}
