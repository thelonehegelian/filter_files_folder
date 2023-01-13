use same_file::is_same_file;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    // get current directory
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());

    // TODO: Fix the error "no file found" on line 15
    // for entry in current_dir.into_iter() {
    // let metadata = fs::metadata(entry).unwrap();
    // let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
    // println!("{:?}", last_modified);
    // }

    /************************
     * DETECT LOOP OF A PATH
     ************************/

    fn contains_loop<P: AsRef<Path>>(path: P) -> io::Result<Option<(PathBuf, PathBuf)>> {
        let path = path.as_ref();
        let mut path_buf = path.to_path_buf();

        while path_buf.pop() {
            if is_same_file(&path_buf, path).unwrap() {
                return Ok(Some((path_buf, path.to_path_buf())));
            } else if let Some(looped_paths) = contains_loop(&path_buf).unwrap() {
                return Ok(Some(looped_paths));
            }
        }
        return Ok(None);
    }

    /******************************************
     * FIND DUPLICATE FILE NAMES AND PRINT THEM
     ******************************************/

    let mut filenames = HashMap::new();
    let mut duplicate_filenames: Vec<String> = Vec::new();
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let filename = entry.file_name().to_string_lossy().to_string();
        let counter = filenames.entry(filename.clone()).or_insert(0);
        *counter += 1;
        // if there is a duplicate, print it
        if *counter == 2 {
            duplicate_filenames.push(filename.clone());
        }
    }
    println!("Duplicate filenames: {:?}", duplicate_filenames);

    /************************************
     * FIND FILES WITHE A GIVEN PREDICATE
     ************************************/

    // TODO: make this into a func
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let sec_modified = entry
            .metadata()
            .unwrap()
            .modified()
            .unwrap()
            .elapsed()
            .unwrap()
            .as_secs();
        if f_name.ends_with(".json") && sec_modified > 100 {
            println!("{} was modified {} seconds ago", f_name, sec_modified);
        }
    }

    /***********************
     * SKIP HIDDEN FILES
     ***********************/

    // TODO: Add hidden_file() function
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.starts_with(".") {
            println!("{}", f_name);
        }
        continue;
    }

    /************************************************
     * calculate sum of all file sizes at given depth
     ************************************************/

    let file_size = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.metadata().unwrap().len())
        .reduce(|x, acc| x + acc)
        .unwrap();
    // .fold(0, |acc, x| acc + x);

    println!("Total file size: {}", file_size);
}
