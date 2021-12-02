use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

use glob::glob;

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

pub fn delete_directory(projects: Vec<PathBuf>) {
    // remove the node_modules of the projects
    for project in projects {
        println!("Removing {}", project.display());
        fs::remove_dir_all(project).unwrap();
    }
}
