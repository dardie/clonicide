//extern crate twox_hash;
#[macro_use]
extern crate clap;
extern crate walkdir;
extern crate crypto;

//use std::collections::HashMap;
use std::path::Path;
use std::process;
use std::io::prelude::*;
use std::io;
use std::io::stdout;
use std::io::BufReader;
use std::fs::File;
//use twox_hash::RandomXxHashBuilder;
use clap::{Arg, App};
use walkdir::WalkDir;
use std::collections::HashMap;

fn get_args() -> String {
    let matches = App::new("dedup")
        .version("1.0")
        .author("Darryl Rees <dardie@gmail.com>")
        .about("Find duplicated or similar folders")
        .arg(Arg::with_name("dir")
            .short("d")
            .long("dir")
            .help("Folder in which to search for duplicate subfolders")
            .multiple(true)
            .takes_value(true)
            .value_name("DIR"))
        .arg(Arg::with_name("match_threshold")
            .required(false)
            .short("m")
            .long("match_threshold")
            .help("Percentage of files within the folders that need to be the same, to report as a duplicate")
            .takes_value(true)
            .value_name("MATCH_THRESHOLD"))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let searchdir = matches.value_of("dir").unwrap_or(".").to_string();
    println!("Folder to search: {}", searchdir);

    let match_threshold = value_t!(matches, "match_threshold", u16).unwrap_or(100);
    println!("Threshold percentage of like files to mark folders as dupes: {}", match_threshold);

    searchdir
}

use crypto::md5::Md5;
use crypto::digest::Digest;

fn hashsum(fpath: &str) -> Result<String, io::Error> {

    let mut hasher = Md5::new();

    const BUFSIZE: usize = 1024 * 4;
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
struct FileInfo {
    dir: String,
    name: String,
    size: u64
}

//static dir_idx: HashMap<String, Option<String>> = HashMap::new();

fn index_by_filehash(filepath: &Path, file_idx: &mut HashMap<String, FileInfo>) {
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
                let info = FileInfo {
                    dir: dir.to_string(),
                    name: name.to_string(),
                    size: metadata.len()
                };
                file_idx.insert (hash.to_string(), info);
            }     
        }       
    }
}

fn main() {
    let d=get_args();
    if !Path::new(&d).is_dir() {
        println!("Specified path '{}' is not a directory", &d);
        process::exit(0);
    } 


    let mut file_idx:HashMap<String, FileInfo> = HashMap::new();
    let mut num_indexed = 0;

    println!("Files Indexed: ");
    for entry in WalkDir::new(&d).into_iter().filter_map(|e| e.ok()) {
        index_by_filehash(entry.path(), &mut file_idx);

        num_indexed = num_indexed + 1;
        if num_indexed % 100 == 0 {
            print!("\r{} : {}                 ", num_indexed, entry.path().to_str().unwrap_or("                         "));
            stdout().flush().expect("Error writing to terminal");
        }
    }
}
