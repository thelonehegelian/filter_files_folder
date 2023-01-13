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
}
