use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// need to add a Result type for better error handling
// Result type that has a proper error handling
fn convert_file_path_to_unix_time(
    trash_path: &Path,
    file: &PathBuf,
    file_name: &OsStr,
) -> Option<PathBuf> {
    // checks to see if file exists in the Trahs folder and if it does creates a new file name with
    // the unix time attached to the file
    if fs::exists(file).unwrap_or(false) {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                let new_path = trash_path.join(format!(
                    "{:}_{:}",
                    file_name.to_str().unwrap_or(""),
                    n.as_secs()
                ));
                return Some(new_path);
            }
            Err(e) => {
                eprint!(
                    "Could not rename file that already exists in Trash folder {}",
                    e
                );
                return None;
            }
        }
    }
    Some(trash_path.join(file_name))
}

fn move_to_trash(trash_path: &Path, file: PathBuf) {
    match file.file_name() {
        Some(file_name) => {
            //println!("Current Path: {:?} Trash path {:?}", file, new_path);

            // checking if the file exists in the Trash folder
            match convert_file_path_to_unix_time(trash_path, &file, file_name) {
                Some(new_path) => match fs::rename(&file, new_path) {
                    Ok(_) => println!("Removed file {file_name:?}"),
                    Err(e) => eprintln!("Failed to move to Trash with {e}"),
                },
                None => {
                    eprint!("Could not move {:?} to Trahs folder", file_name)
                }
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
