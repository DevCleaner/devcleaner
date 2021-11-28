use std::fs;
use std::path::PathBuf;

pub fn find_directories(path: &str, criteria: &str) -> Vec<PathBuf> {
    let mut directories = vec![];
    // search recursively for directories "node_modules" and push them to the vector
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let path = entry.path();
            if path.ends_with(criteria) {
                directories.push(path);
            } else {
                directories.append(&mut find_directories(&path.to_str().unwrap(), criteria));
            }
        }
    }
    directories
}


pub fn delete_directory(projects: &mut Vec<PathBuf>) {
// remove the node_modules of the projects
    for project in projects {
        println!("Removing {}", project.display());
        fs::remove_dir_all(project).unwrap();
    }
}