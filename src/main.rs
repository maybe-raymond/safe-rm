use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please enter the files you want to put in the Trash file");
        return;
    }

    let trash_path = safe_rm::get_trash_path();
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
        safe_rm::process_content_for_deletion(trash_path, content);
    }
}
