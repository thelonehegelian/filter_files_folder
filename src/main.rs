use same_file::is_same_file;
use std::io;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    // get current directory
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());

    for entry in current_dir.into_iter() {
        let metadata = fs::metadata(entry).unwrap();
        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
        println!("{:?}", last_modified);
    }

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
}
