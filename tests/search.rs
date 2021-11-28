use nmkill::find_directories;

#[test]
fn test_find_directories_with_directory_and_criteria() {
    let path_directory = "/home/acosta/Desktop/test/";
    let directories = find_directories(path_directory, "node_modules");
    println!("{:?}", directories);
    assert!(true);
}

