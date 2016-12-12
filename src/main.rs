#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;
extern crate walkdir;
extern crate crypto;
mod hash_index;

use std::process;
use std::io::prelude::*;
use std::io::stdout;
use clap::{Arg, App};
use walkdir::WalkDir;
use std::collections::BTreeMap;
use std::path::Path;
use hash_index::{FileInfo, FileList, HashIndex, HasAddFile};
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

#[derive(Eq,PartialEq,PartialOrd,Ord)]
// The 'Value' for MatchedPairIndex
struct MatchedFilePair {
    hash: String,
    dir1_names: String, // change to vec of strings, files with same hash within dir
    dir2_names: String // change to vec of strings
}

// The 'Key' for MatchedPairIndex
type MatchedDirPair = (String, String); //(Dir1, Dir2);
//static dir_idx: HashMap<String, Option<String>> = HashMap::new();

fn add_matched_pair(file_hash:&str, file1: &FileInfo, file2: &FileInfo, matched_pair_idx: &mut BTreeMap<MatchedDirPair, MatchedFilePair>) {
    let key = (file1.dir.clone(), file2.dir.clone());
    let val = MatchedFilePair {
        hash: file_hash.to_string(),
        dir1_names: file1.name.clone(),
        dir2_names: file2.name.clone()
    };
    matched_pair_idx.insert(key, val);
}

fn main() {
    env_logger::init().unwrap();
    let d=get_args();
    if !Path::new(&d).is_dir() {
        println!("Specified path '{}' is not a directory", &d);
        process::exit(0);
    } 

    let mut file_idx:HashIndex = HashMap::new();
    let mut num_indexed = 0;

    info!("Files Indexed: ");
    for entry in WalkDir::new(&d).into_iter().filter_map(|e| e.ok()) {
        file_idx.add_file(entry.path());

        num_indexed = num_indexed + 1;
        if num_indexed % 100 == 0 {
            debug!("\r{} : {}                 ", num_indexed, entry.path().to_str().unwrap_or("                         "));
            stdout().flush().expect("Error writing to terminal");
        }
    }

    info!("Collating folders..");
    let mut matched_pair_idx = BTreeMap::new();
    for (hashkey, file_list) in file_idx.iter().filter(|&(_,v)| v.len() >= 2) {
        debug!("Adding hash {:?} file1 {:?} file2 {:?} to matched pair idx", &hashkey, &file_list[0], &file_list[1] );
        add_matched_pair(&hashkey, &file_list[0], &file_list[1], &mut matched_pair_idx);
    }
}
