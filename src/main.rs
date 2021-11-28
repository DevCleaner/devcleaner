use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path_directory = &args[1];

    println!("Searching for {}", path_directory);

    let files = fs::read_dir(path_directory).unwrap();
    // search recursively in the directory for node_modules and save the path of the project in a vector
    let mut projects = vec![];
    for file in files {
        let file = file.unwrap();
        // if the file is a directory search recursively in the directory for node_modules and save the path of the project in a vector
        if file.file_type().unwrap().is_dir() {
            let path = file.path();
            let path_node_modules = path.join("node_modules");
            if path_node_modules.exists() {
                projects.push(path_node_modules);
            }
            // if the file is a directory search recursively in the directory for node_modules and save the path of the project in a vector
            let files_directory = fs::read_dir(path).unwrap();
            for file_directory in files_directory {
                let file_directory = file_directory.unwrap();
                let path_directory = file_directory.path();
                let path_node_modules = path_directory.join("node_modules");
                if path_node_modules.exists() {
                    projects.push(path_node_modules);
                }
            }
        }
    }
    println!("Found {} projects", projects.len());
    // remove the node_modules of the projects
    for project in projects {
        println!("Removing {}", project.display());
        fs::remove_dir_all(project).unwrap();
    }
}

