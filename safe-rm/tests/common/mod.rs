use std::fs;
use std::path::PathBuf;

// once created auto deletes everything in the folder
pub struct TempFolder {
    pub path: PathBuf,
}

// delete the folder and all it's files once it is done
impl Drop for TempFolder {
    fn drop(&mut self) {
        match self.path.try_exists() {
            Err(_) => println!("File already deleted"),
            Ok(false) => println!("File does not exist"),
            Ok(true) => {
                let _ = fs::remove_dir_all(&self.path);
            }
        };
    }
}

// creating a temporary trash bin
pub fn setup(folder_name: &str) -> Option<TempFolder> {
    match fs::create_dir(folder_name) {
        Ok(_) => Some(TempFolder {
            path: PathBuf::from(folder_name),
        }),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}
