use std::fs;
use std::io::Write;

use nmkill::{delete_directory, find_directories};

fn create_directories_for_tests() {
    let base = String::from("directory_tests");
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
            rand::random::<u8>()
        ));
    }

    for i in 0..directory_tree.len() {
        // create a directory tree example for tests
        let mut dir = std::env::current_dir().unwrap();
        let path = directory_tree[i].clone();

        // check if path end with extension
        if path.ends_with(".js") || path.ends_with(".html") {
            dir.push(path);
            fs::create_dir_all(dir.parent().unwrap()).unwrap();
            fs::File::create(dir.as_path()).unwrap();
            // write content to file
            let mut file = fs::OpenOptions::new()
                .write(true)
                .open(dir.as_path())
                .unwrap();
            file.write_all(b"test").unwrap();
        } else {
            dir.push(path);
            fs::create_dir_all(dir.as_path()).unwrap();
        }
        dir.pop();
    }
}

fn delete_directory_tree_after_end_the_tests() {
    let base = String::from("directory_tests");
    let mut dir = std::env::current_dir().unwrap();
    dir.push(base);
    fs::remove_dir_all(dir.as_path()).unwrap();
}

#[test]
fn test_find_directories_with_directory_and_criteria() {
    create_directories_for_tests();
    let path_directory = "directory_tests";
    let directories = find_directories(path_directory, "node_modules");
    println!("{:?}", directories);
    // check if the number of directories is correct
    assert_eq!(directories.len(), 12);
    delete_directory_tree_after_end_the_tests();
}

