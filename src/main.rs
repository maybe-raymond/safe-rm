use std::env;
use std::fs;
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
            //Getting the absolute path for all the files
            let all_files = folder_list.iter().map(fs::canonicalize);

            // looping through all the files and moving them to Trash one by one
            for file_path in all_files {
                match file_path {
                    Ok(path) => {
                        // This might not be nessary since move to trah moved the whole project including the subfolders
                        // Might just need to expirement with it

                        move_to_trash(&trash_location, path)
                    }
                    Err(e) => eprintln!("Program exited with error: {:}", e),
                }
            }
        }
        _ => {
            // Already checked if the len of the args are bigger than one
            let (_, files) = args.split_at(1);

            //Getting the absolute path for all the files
            let all_files = files.iter().map(fs::canonicalize);

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
