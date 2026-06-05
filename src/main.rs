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
fn get_trash_path() -> PathBuf {
    // Getting the location of the Trash folder on linux
    let home_directoy = env::home_dir().expect("Cannot find the home direcory for this os");
    home_directoy.join(".local/share/Trash/files/")
}

fn process_content_for_deletion(trash_path: &Path, content: &[String]) {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please enter the files you want to put in the Trash file");
        return;
    }

    let trash_path = get_trash_path();
    let trash_path = trash_path.as_path();

    if args[1].as_str() == "-h" {
        println!(
            "Safe-rm is a safe alternative to rm command. It outputs all removed files to the Trash folder."
        );
    } else {
        let mid = if args[1].as_str() == "-r" { 2 } else { 1 }; // determines when to cut the args. 1 for files and 2 for folders
        let (_, content) = args.split_at(mid);

        if content.is_empty() {
            println!("No files or folders were input for deletetion");
            return;
        }
        process_content_for_deletion(trash_path, content);
    }
}
