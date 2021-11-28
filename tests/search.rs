use std::fs;
use std::io::Write;

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
            rand::random::<u8>()
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

#[cfg(test)]
mod tests {
    use nmkill::{delete_directory, find_directories};

    use crate::{create_directories_for_tests, delete_directory_tree_after_end_the_tests};

    #[test]
    fn test_find_directories_with_directory_and_criteria() {
        create_directories_for_tests(String::from("test_find_directories_with_directory_and_criteria"));
        let path_directory = "test_find_directories_with_directory_and_criteria";
        let directories = find_directories(path_directory, "node_modules");
        println!("{:?}", directories);
        // check if the number of directories is correct
        assert_eq!(directories.len(), 12);
        delete_directory_tree_after_end_the_tests(String::from("test_find_directories_with_directory_and_criteria"));
    }

    #[test]
    fn test_delete_directories_with_criteria() {
        create_directories_for_tests(String::from("test_delete_directories_with_criteria"));
        let path_directory = "test_delete_directories_with_criteria";
        let directories = find_directories(path_directory, "node_modules");
        println!("{:?}", directories);
        // check if the number of directories is correct
        assert_eq!(directories.len(), 12);
        delete_directory(directories);
        let directories = find_directories(path_directory, "node_modules");
        println!("{:?}", directories);
        // check if the number of directories is correct
        assert_eq!(directories.len(), 0);
        delete_directory_tree_after_end_the_tests(String::from("test_delete_directories_with_criteria"));
    }
}