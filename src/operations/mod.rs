use std::fs;
use std::path::PathBuf;

use glob::glob;
use rayon::prelude::*;

use crate::app::project::Project;

pub fn find_projects(path: &str, criteria: &str) -> Vec<Project> {
    let projects = find_directories(path, criteria);
    let mut projects: Vec<Project> = projects.into_iter().map(Project::new).collect();

    projects.par_sort_unstable_by(|a, b| a.name.cmp(&b.name));
    projects
}

pub fn find_directories(path: &str, criteria: &str) -> Vec<PathBuf> {
    let mut directories: Vec<PathBuf>;
    let criteria = criteria.split(',').collect::<Vec<&str>>();
    let mut p = path.to_string();
    if !p.ends_with('/') {
        p.push('/');
    }
    // https://docs.rs/globwalk/latest/globwalk/
    directories = criteria
        .par_iter()
        .map(|criterion| {
            let mut path = p.clone();
            let c = criterion.trim();
            let c = "**/".to_owned() + c;
            path.push_str(c.as_str());
            glob(path.as_str())
                .expect("Failed to read glob pattern")
                .filter(|entry| entry.is_ok())
                .map(|entry| entry.unwrap())
                .collect::<Vec<PathBuf>>()
        })
        .flatten()
        .collect::<Vec<PathBuf>>();

    // remove duplicates
    directories.sort();
    directories.dedup();
    // keep only the parent directories
    // know if the path is parent of the current directory
    let mut is_parent = vec![false; directories.len()];
    for i in 0..directories.len() {
        for j in 0..directories.len() {
            if i == j {
                continue;
            }
            if directories[i].starts_with(&directories[j]) {
                is_parent[i] = true;
            }
        }
    }
    let mut result = vec![];
    for i in 0..directories.len() {
        if !is_parent[i] {
            result.push(directories[i].clone());
        }
    }
    result
}

pub fn delete_directory(path: &str) {
    fs::remove_dir_all(path).expect("Failed to delete directory");
}

pub fn delete_directories(directories: Vec<PathBuf>) {
    // remove the node_modules of the projects
    for dir in directories {
        fs::remove_dir_all(dir).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    use rand::random;

    use crate::operations::{delete_directories, find_directories};

    // test for the operations module
    fn create_directories_for_tests(base: String) {
        let mut directory_tree: Vec<String> = vec![
            base.clone(),
            base.clone() + "/index.html",
            base.clone() + "/node_modules",
            base.clone() + "/node_modules/index.html",
        ];
        for i in 0..=10 {
            directory_tree.push(format!(
                "{}/Project-{}/node_modules/test_file_{}.js",
                base,
                i,
                random::<u8>()
            ));
        }

        for directory in &directory_tree {
            // create a directory tree example for tests
            let mut dir = std::env::current_dir().unwrap();
            let path = directory.clone();
            dir.push(&path);
            // check if path end with extension
            if path.ends_with(".js") || path.ends_with(".html") {
                fs::create_dir_all(dir.parent().unwrap()).unwrap();
                fs::File::create(dir.as_path()).unwrap();
                // write content to file
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .open(dir.as_path())
                    .unwrap();
                file.write_all(b"test").unwrap();
            } else {
                fs::create_dir_all(dir.as_path()).unwrap();
            }
            dir.pop();
        }
    }

    fn delete_directory_tree_after_end_the_tests(base: String) {
        let mut dir = std::env::current_dir().unwrap();
        dir.push(base);
        fs::remove_dir_all(dir.as_path()).unwrap();
    }

    #[test]
    fn test_find_directories_with_directory_and_criteria() {
        const FOLDER: &str = "test_find_directories_with_directory_and_criteria";
        create_directories_for_tests(String::from(FOLDER));
        let path_directory = FOLDER;
        let directories = find_directories(path_directory, "node_modules");
        println!("{:?}", directories);
        // check if the number of directories is correct
        assert_eq!(directories.len(), 12);
        delete_directory_tree_after_end_the_tests(String::from(FOLDER));
    }

    #[test]
    fn test_delete_directories_with_criteria() {
        const FOLDER: &str = "test_delete_directories_with_criteria";
        create_directories_for_tests(String::from(FOLDER));
        let path_directory = FOLDER;
        let directories = find_directories(path_directory, "node_modules");
        println!("{:?}", directories);
        // check if the number of directories is correct
        assert_eq!(directories.len(), 12);
        delete_directories(directories);
        let directories = find_directories(path_directory, "node_modules");
        println!("{:?}", directories);
        // check if the number of directories is correct
        assert_eq!(directories.len(), 0);
        delete_directory_tree_after_end_the_tests(String::from(FOLDER));
    }

    #[test]
    fn test_delete_directories_with_multiples_criteria() {
        const FOLDER: &str = "test_delete_directories_with_multiples_criteria";
        create_directories_for_tests(String::from(FOLDER));
        let path_directory = FOLDER;
        let directories = find_directories(path_directory, "node_modules, project1");
        println!("{:?}", directories);
        // check if the number of directories is correct
        assert_eq!(directories.len(), 12);
        delete_directories(directories);
        let directories = find_directories(path_directory, "node_modules");
        println!("{:?}", directories);
        // check if the number of directories is correct
        assert_eq!(directories.len(), 0);
        delete_directory_tree_after_end_the_tests(String::from(FOLDER));
    }
}
