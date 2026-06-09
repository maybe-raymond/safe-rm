mod common;

use std::fs;
use std::path::Path;

#[test]
fn delete_one_file() {
    // deletes one file
    let file_name = "code_base.py";
    let trash_path = common::setup().unwrap();
    let _file = fs::File::create(file_name).unwrap();

    let new_path = Path::new(file_name);
    safe_rm::process_content_for_deletion(trash_path.as_path(), &[String::from(file_name)]);

    assert!(!new_path.exists())
}

#[test]
fn delete_one_folder() {
    // deletes one file
}

#[test]
fn delete_multiple_folders() {
    // deletes one file
}

#[test]
fn delete_multiple_files() {
    // deletes one file
}

#[test]
fn try_delete_file_that_does_exist() {
    // deletes one file
}
