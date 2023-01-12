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
}
