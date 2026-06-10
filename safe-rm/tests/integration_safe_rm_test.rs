mod common;

use std::fs;
use std::path::Path;

use safe_rm::process_content_for_deletion;

#[test]
fn delete_one_file() {
    // deletes one file
    let file_name = "code_base.py";
    let trash_path = common::setup("trash_delete_one_file").unwrap();
    let _file = fs::File::create(file_name).unwrap();

    let new_path = Path::new(file_name);
    safe_rm::process_content_for_deletion(trash_path.path.as_path(), &[String::from(file_name)]);

    assert!(!new_path.exists() && trash_path.path.join(file_name).exists())
}

#[test]
fn delete_one_folder() {
    // deletes a folder and all it's contents/
    // checks to see if temp folder is in trash path
    // checks to see if the deleted files are in the new trash folder
    let trash_path = common::setup("trash_delete_one_folder").unwrap();
    let temp_folder = common::setup("temp_folder").unwrap();

    // create 5 files in temp directory
    (0..5).for_each(|x| {
        fs::File::create(temp_folder.path.join(format!("temp_{x}"))).unwrap();
    });

    safe_rm::process_content_for_deletion(
        trash_path.path.as_path(),
        &[String::from("temp_folder")],
    );

    // get the amount of files in the trash
    let trash_amount = fs::read_dir(&trash_path.path).unwrap().count();
    let files_amount = fs::read_dir(trash_path.path.join("temp_folder"))
        .unwrap()
        .count();

    // check if the folder still exists and trash has 4 files
    let exists = temp_folder.path.try_exists().unwrap_or(false);
    assert!(!exists && trash_amount == 1 && files_amount == 5)
}

#[test]
fn delete_multiple_files() {
    // deletes one file
    let trash_path = common::setup("trash_delete_mut_files").unwrap();

    // create 5 files in temp directory
    let file_names = (0..5).map(|x| format!("temp_{x}"));

    // creates Files
    file_names.clone().for_each(|i| {
        fs::File::create(i).unwrap();
    });

    let file_list = &file_names.clone().collect::<Vec<_>>();
    process_content_for_deletion(trash_path.path.as_path(), file_list);

    assert!(file_names.clone().all(|x| !fs::exists(x).unwrap_or(false)));
}

#[test]
fn delete_multiple_folders() {
    // deletes one file
    todo!()
}

/*
* Can not do tests on deleting a file or folder that does not exist since
* process_content_for_deletion does not return an Result type
*
*
#[test]
#[should_panic]
fn try_delete_file_that_does_not_exist() {
    // tries to delete a file that does not exist
    let file_name = "code_base.py";
    let trash_path = common::setup("trash_delete_one_file").unwrap();

    safe_rm::process_content_for_deletion(trash_path.path.as_path(), &[String::from(file_name)]);
}

#[test]
#[should_panic]
fn try_delete_folder_that_does_not_exist() {
    // tries to delete a folder that does not exist
    todo!()
}

*/
