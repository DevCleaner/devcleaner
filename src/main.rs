use nmkill::{delete_directory, find_directories};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path_directory = &args[1];

    println!("Searching for {}", path_directory);

    let projects = find_directories(path_directory, "node_modules");

    println!("Found {} projects", projects.len());
    delete_directory(projects)
}
