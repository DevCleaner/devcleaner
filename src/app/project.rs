use std::path::PathBuf;

use crate::app::utilities::dir_size;

#[derive(Clone, PartialEq, Debug)]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
    pub mark_for_deletion: bool,
    pub is_deleted: bool,
    pub size: u64,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            path: PathBuf::new(),
            name: String::new(),
            mark_for_deletion: false,
            is_deleted: false,
            size: 0,
        }
    }
}

impl Project {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let mut project = Project {
            path,
            name,
            mark_for_deletion: false,
            is_deleted: false,
            size: 0,
        };
        project.size = project.calculate_size();
        project
    }
    pub fn calculate_size(&self) -> u64 {
        dir_size(self.path.clone()).unwrap_or(0)
    }
}
