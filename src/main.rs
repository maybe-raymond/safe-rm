use std::env;
use std::fs;
use std::path::PathBuf;

// need to implement for fuzzy search

fn move_to_trash(trash_path: &PathBuf, file: PathBuf) {
    match file.file_name() {
        Some(file_name) => {
            let new_path = trash_path.join(file_name);
            //println!("Current Path: {:?} Trash path {:?}", current_path, new_path);

            match fs::rename(&file, new_path) {
                Ok(_) => println!("Removed file {file_name:?}"),
                Err(e) => eprintln!("Faild to move to Trash with {e}"),
            }
        }
        None => println!("File name for file {file:?} not found"),
    }
}

fn delete_files_in_path(folder_contents: fs::ReadDir, trash_path: &PathBuf) {
    for file in folder_contents {
        match file {
            Ok(file) => {
                let file_path = file.path();
                if file_path.is_file() {
                    move_to_trash(trash_path, file_path);
                } else {
                    println!("{:?} is a sub direcotry", file_path);
                    let contents = file_path
                        .read_dir()
                        .expect("Could not read the contents of folder");
                    delete_files_in_path(contents, trash_path);
                }
            }
            Err(e) => eprintln!("Error {e:?} occured while trying to delete file"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please enter the files you want to put in the Trash file");
        return;
    }

    // Getting the location of the Trash folder on linux
    let home_directoy = env::home_dir().expect("Cannot find the home direcory for this os");
    let trash_location = home_directoy.join(".local/share/Trash/files/");

    match args[1].as_str() {
        "-h" => println!(
            "Safe-rm is a safe alternative to rm command. It outputs all removed files to the Trash folder."
        ),
        "-r" => {
            let folder = args
                .get(2)
                .expect("Folder to be removed was not added to the command args");

            let folder_path = PathBuf::from(folder);

            let folder_dir = if folder_path.is_absolute() {
                folder_path
            } else {
                let cwd_directory = env::current_dir()
                    .expect("Cannot Find the current working directory for this os");
                cwd_directory.join(&folder)
            };

            let folder_contents = folder_dir
                .read_dir()
                .expect("Could not read the contents of folder");
            delete_files_in_path(folder_contents, &trash_location);
        }
        _ => {
            // Getting the current working directory of the program
            let cwd_directory =
                env::current_dir().expect("Cannot Find the current working directory for this os");

            // looping through all the files and moving them to Trash one by one
            for file_path in &args[1..] {
                let file = PathBuf::from(file_path);

                if file.is_absolute() {
                    move_to_trash(&trash_location, file)
                } else if file.is_file() {
                    let full_path = cwd_directory.join(file);
                    move_to_trash(&trash_location, full_path)
                } else {
                    eprintln!("{file_path} might be a directory or might not exist");
                }
            }
        }
    }
}
