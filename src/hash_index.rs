use crypto::md5::Md5; //Try Seahash, and use .result instead of .result_str
use crypto::digest::Digest;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn hashsum(fpath: &str) -> Result<String, io::Error> {
    let mut hasher = Md5::new();
    const BUFSIZE: usize = 1024 * 4; // Empirically, faster than 1K and 16K
    let file = File::open(fpath)?;
    let mut reader = BufReader::with_capacity(BUFSIZE, file);

    loop {
        let length = {
            let buffer = reader.fill_buf()?;
            hasher.input(buffer);
            buffer.len()
        };
        if length == 0 { break; }
        reader.consume(length);
    }
    let hash = hasher.result_str();
//    let hash = "blah".to_string();
    Ok(hash)
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct FileInfo {
    pub dir: String,
    pub name: String,
    pub size: u64
}

pub type FileList = Vec<FileInfo>;

pub type HashIndex = HashMap<String, FileList>;

pub trait HasAddFile {
    fn add_file(&mut self, filepath: &Path);
}

impl HasAddFile for HashIndex {
    fn add_file(&mut self, filepath: &Path) {
        if let (
            Some(path),
            Some(dir),
            Some(name),
            Ok(metadata)
        ) = (
            filepath.to_str(), // path
            filepath.parent().and_then( |p| p.to_str() ), //dir
            filepath.file_name().and_then( |p| p.to_str() ), //name
            filepath.metadata() //metadata
        )
        {
            if metadata.is_file() {
                if let Ok(hash) = hashsum(path) {
                    let file_rec = FileInfo {
                        dir: dir.to_string(),
                        name: name.to_string(),
                        size: metadata.len()
                    };
                    let files_matching_hash:&mut FileList = self.entry(hash).or_insert( Vec::new() );
                    files_matching_hash.push(file_rec);
                }     
            }       
        }
    }
}

