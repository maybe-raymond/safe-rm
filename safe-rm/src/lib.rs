use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn move_to_trash(trash_path: &Path, file: PathBuf) {
    match file.file_name() {
        Some(file_name) => {
            let new_path = trash_path.join(file_name);
            //println!("Current Path: {:?} Trash path {:?}", file, new_path);

            match fs::rename(&file, new_path) {
                Ok(_) => println!("Removed file {file_name:?}"),
                Err(e) => eprintln!("Faild to move to Trash with {e}"),
            }
        }
        None => println!("File name for file {file:?} not found"),
    }
}

// need to make this into an Option type so it does not crash
pub fn get_trash_path() -> PathBuf {
    // Getting the location of the Trash folder on linux
    let home_directoy = env::home_dir().expect("Cannot find the home direcory for this os");
    home_directoy.join(".local/share/Trash/files/")
}

pub fn process_content_for_deletion(trash_path: &Path, content: &[String]) {
    // purely exists to make testing easier and breaking up logic from main

    let absolute_content = content.iter().map(fs::canonicalize); // Getting the absolute path for all the files or folders

    // looping through all the files and moving them to Trash one by one
    for item in absolute_content {
        match item {
            Ok(path) => move_to_trash(trash_path, path),
            Err(e) => eprintln!("Program exited with error: {:}", e),
        }
    }
}

