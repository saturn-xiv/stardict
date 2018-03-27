use std::{fs, io, path};
use std::io::BufRead;

use super::result::Result;

#[derive(Debug, Clone)]
pub struct Ifo {
    pub author: String,
    pub version: String,
    pub name: String,
    pub date: String,
    pub description: String,
    pub email: String,
    pub web_site: String,
    pub same_type_sequence: String,
    pub idx_file_size: isize,
    pub word_count: isize,
    pub syn_word_count: isize,
}

impl Ifo {
    pub fn open(file: path::PathBuf) -> Result<Ifo> {
        let mut it = Ifo {
            author: String::new(),
            version: String::new(),
            name: String::new(),
            date: String::new(),
            description: String::new(),
            email: String::new(),
            web_site: String::new(),
            same_type_sequence: String::new(),
            idx_file_size: 0,
            word_count: 0,
            syn_word_count: 0,
        };
        for line in io::BufReader::new(try!(fs::File::open(file))).lines() {
            let line = try!(line);
            if let Some(id) = line.find('=') {
                let key = &line[..id];
                let val = String::from(&line[id + 1..]);
                match key {
                    "author" => it.author = val,
                    "bookname" => it.name = val,
                    "version" => it.version = val,
                    "description" => it.description = val,
                    "date" => it.date = val,
                    "idxfilesize" => it.idx_file_size = try!(val.parse()),
                    "wordcount" => it.word_count = try!(val.parse()),
                    "website" => it.web_site = val,
                    "email" => it.email = val,
                    "sametypesequence" => it.same_type_sequence = val,
                    "synwordcount" => it.syn_word_count = try!(val.parse()),
                    _ => warn!("Ingnore line: {}", line),
                };
            }
        }
        return Ok(it);
    }
}
