use std::fs;
use std::path::PathBuf;

// creating a temporary trash bin
pub fn setup() -> Option<PathBuf> {
    let path = "trash";
    match fs::create_dir(path) {
        Ok(_) => Some(PathBuf::from(path)),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}
