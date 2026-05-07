use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

fn move_to_trash(trash_path: &PathBuf, file: PathBuf) {
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

fn delete_files_in_path(folder_contents: fs::ReadDir, trash_path: &PathBuf) {
    // This function needs to handle creating new subdirectories and deleting them
    for file in folder_contents {
        match file {
            Ok(file) => {
                let file_path = file.path();
                if file_path.is_file() {
                    move_to_trash(trash_path, file_path);
                } else {
                    let contents = file_path
                        .read_dir()
                        .expect("Could not read the contents of folder");
                    // create new subdirectory and append it to the trash folder then add the new one
                    delete_files_in_path(contents, trash_path);
                    fs::remove_dir(file_path)
                        .expect("Failed to delete folder. Please check your permissions");
                }
            }
            Err(e) => eprintln!("Error {e:?} occured while trying to delete file"),
        }
    }
}

fn create_file_directory(folder_path: &PathBuf) -> Result<&PathBuf, std::io::Error> {
    // creates a dir and returns the pathBuf, the create_dir function returns an error when the folder exists but we just want the path to that folder either way
    // So this is just a wrapper to get the behaviour we desire
    match fs::create_dir(&folder_path) {
        Ok(_) => Ok(folder_path),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => Ok(folder_path),
            _ => Err(e),
        },
    }
}

fn process_folder_deletion(trash_path: &PathBuf, folder_name: &OsStr, folder_path: &PathBuf) {
    // creating a folder in trash path to push all the new items
    match create_file_directory(&trash_path.join(folder_name)) {
        Ok(path) => {
            //getting files in the folder
            // Will fail if the path is a file
            let folder_contents = folder_path
                .read_dir()
                .expect("Could not read the contents of folder");

            delete_files_in_path(folder_contents, &path);
            // deleting folder
            fs::remove_dir(folder_path)
                .expect("Failed to delete folder. Please check your permissions");
        }
        Err(e) => eprintln!("{}", e),
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
            // meeant for folders
            let (_, folder_list) = args.split_at(2);

            if folder_list.len() < 2 {
                println!("Please add folders to delete");
                return;
            }
            let folders = folder_list.into_iter().map(fs::canonicalize);

            for folder in folders {
                match folder {
                    Ok(folder_path) => {
                        if let Some(folder_name) = folder_path.file_name() {
                            process_folder_deletion(&trash_location, folder_name, &folder_path);
                        } else {
                            eprintln!("selected directory does not have a name");
                        }
                    }
                    Err(e) => eprint!("Could not resolve path with error: {}", e),
                }
            }
        }
        _ => {
            // Already checked if the len of the args are bigger than one
            let (_, files) = args.split_at(1);

            //Getting the absolute path for all the files
            let all_files = files.into_iter().map(fs::canonicalize);

            // looping through all the files and moving them to Trash one by one
            for file_path in all_files {
                match file_path {
                    Ok(path) => {
                        // This might not be nessary since move to trah moved the whole project including the subfolders
                        // Might just need to expirement with it
                        if path.is_file() {
                            move_to_trash(&trash_location, path)
                        } else {
                            println!("{:?} is a direcory, please use the -r command", path)
                        }
                    }
                    Err(e) => eprintln!("Program exited with error: {:}", e),
                }
            }
        }
    }
}
