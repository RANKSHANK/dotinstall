use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use dirs;
use symlink;

const IGNORE: [&str; 4] = [".git", ".gitignore", "README.md", "dotinstall"];

fn main(){
    walk_directory(
        &env::current_dir().expect("Couldn't accertain CWD").as_path(),
        &dirs::home_dir().expect("Couldn't accetain home"),
    );
}

// Walks the provided path, recursively following branches 
fn walk_directory( dir:  &Path, home : &PathBuf) -> std::io::Result<()>{
    for entry in fs::read_dir(dir)?{
        let entry = entry.unwrap();
        // Filter out entries matching the global ignore
        if let Some(file_name) = entry.path().file_name() {
            if let Some(name_str) = file_name.to_str() {
                if IGNORE.iter().any(|&f| f == name_str) {
                    continue;
                }
            }
        }
        let mut target = home.clone();
        target.push(entry.file_name());
 
        if target.exists() {
            match target.metadata() {
                Ok(metadata) => {
                    // Remove existing symlinks so they can be replaced
                    if metadata.is_symlink() {
                        if metadata.is_dir() {
                            symlink::remove_symlink_dir(&target);
                        } else {
                            symlink::remove_symlink_file(&target);
                        }
                    } else {
                        // Recurse directories, ignore files
                        if metadata.is_dir()  {
                            walk_directory(entry.path().as_path(), &target);
                        }
                        continue;
                    }
                },
                Err(err) => {
                    println!("Unable to grab DirEntry's metadata {}", err);
                }
            }
        }
        // Generate symlinks
        match entry.metadata(){
            Ok(metadata) => {
                if metadata.is_dir() {
                    symlink::symlink_dir(entry.path(), target);
                } else if metadata.is_file() {
                    symlink::symlink_file(entry.path(), target);
                }
            },
            Err(err) => {
                println!("Unable to grab DirEntry's metadata {}", err);
            },
        }
    }
    Ok(())
}

