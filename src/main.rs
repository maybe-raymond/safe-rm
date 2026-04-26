use std::env;
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
                    delete_files_in_path(contents, trash_path);
                    fs::remove_dir(file_path)
                        .expect("Failed to delete folder. Please check your permissions");
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

            // creating a folder in trash path to push all the new items
            let folder_name = folder_dir
                .file_name()
                .expect("seletect DIR does not have a name");
            let trash_location = trash_location.join(folder_name);

            match fs::create_dir(&trash_location) {
                Ok(_) => {
                    //println!("Folder {:?} was created", trash_location);
                    delete_files_in_path(folder_contents, &trash_location);
                    // deleting folder
                    fs::remove_dir(folder_dir)
                        .expect("Failed to delete folder. Please check your permissions");
                }
                Err(e) => match e.kind() {
                    ErrorKind::AlreadyExists => {
                        // keep going and delete the file
                        //println!("Folder already exiosts");
                        delete_files_in_path(folder_contents, &trash_location);
                        // deleting folder
                        fs::remove_dir(folder_dir)
                            .expect("Failed to delete folder. Please check your permissions");
                    }
                    _ => eprintln!("{}", e),
                },
            }
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
