use std::collections::HashMap;
use std::fs::{File, read_dir};
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

  println!("File contents: {:?}", ignored_files);
  println!("Dirs: {:?}", dirs);
}

fn get_gitignore_map() -> HashMap<String, bool> {
  let file = File::open(".gitignore").unwrap();
  let reader = BufReader::new(file);
  let mut map = HashMap::new();

  for line in reader.lines() {
    let key = line.unwrap();
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
      if path.to_string_lossy().contains("node_modules") || path.to_string_lossy().contains(".git")
      {
        continue;
      }
      // println!("{:?}", relative_file_path);
      // println!(
      //   "{:?}",
      //   path
      //     .read_dir()
      //     .into_iter()
      //     .for_each(|s| println!("{:?}", s))
      // );
      
      if let Ok(entries) = read_dir(path) {
        for entry in entries {
          if let Ok(entry) = entry {
            // Here, `entry` is a `DirEntry`.
            println!("{:?}", entry.file_name());
          }
        }
      }
      let parts = relative_file_path.split('/');

      // file_name.
      if let Some(last) = parts.last() {
        if ignored_files.contains_key(last) {
          println!("{:?}", last);
        }
      }

      dirs.push(entry.path().to_string_lossy().to_string());
    }
  }
  dirs
}
