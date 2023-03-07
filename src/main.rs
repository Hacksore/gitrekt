use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};

use walkdir::WalkDir;

// clean the the input to remove comments in the gitignore + whitespace
// read the gitignore into a map for quick lookup
// if the folder contains a non-git ignored file we can continue/skip to next folder
// if the current folder does not have any non ignored files we can delete the folder

/* possible data structure:
node_modules
dist
*/

fn main() {
    let current_dir = ".";
    let ignored_files = get_gitignore_map();
    let dirs = get_all_dirs(current_dir, &ignored_files);

    println!("Dirs: {:?}", dirs);
}

fn get_gitignore_map() -> HashMap<String, bool> {
    let file = File::open(".gitignore").unwrap();
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        let key = line.unwrap().replace('/', "");
        map.insert(key, true);
    }

    map
}

fn get_all_dirs(dir_path: &str, ignored_files: &HashMap<String, bool>) -> Vec<String> {
    let mut dirs = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let mut relative_file_path = path.to_string_lossy().to_string();

        if relative_file_path != "." {
            relative_file_path.remove(0);
            relative_file_path.remove(0);
        } else {
            continue;
        }

        if entry.path().is_dir() {
            // TODO: fix later
            if path.to_string_lossy().contains("node_modules")
                || path.to_string_lossy().contains(".git")
            {
                continue;
            }

            let parts = relative_file_path.split('/');

            if let Some(last) = parts.last() {
                // check to see if directory is ignored
                if ignored_files.contains_key(last) {
                    // if directory is ignored we check to see if any sibling directories or files are not ignored
                    if path
                        .parent()
                        .unwrap()
                        .to_string_lossy()
                        .contains("node_modules")
                        || path.parent().unwrap().to_string_lossy().contains(".git")
                        || path.parent().unwrap().to_string_lossy().eq(".")
                    {
                        continue;
                    } else if let Ok(entries) = read_dir(path.parent().unwrap()) {
                        let mut sib = Vec::new();
                        for entry in entries.flatten() {
                            // if the parent contains any non-ignored files we push to vec and continue
                            if entry.path().is_file()
                                && !ignored_files
                                    .contains_key(&entry.file_name().to_string_lossy().to_string())
                            {
                                sib.push(path.to_string_lossy().to_string());
                            } else {
                                // if no siblings are not ignored we can delete the directory
                                continue;
                            }
                        }
                        if sib.is_empty() {
                            dirs.push(path.parent().unwrap().to_string_lossy().to_string());
                        }
                    }
                } else {
                    // if directory is not ignored we continue
                    continue;
                }
            }
        } else {
            continue;
        }
    }
    dirs
}
