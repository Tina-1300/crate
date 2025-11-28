use std::fs::{self, remove_file, remove_dir_all};
use std::path::{Path, PathBuf};


pub fn collect_paths(dir: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(dir){
        for entry in entries.flatten(){
            let path = entry.path();
            if path.is_dir(){
                paths.extend(collect_paths(&path));
                paths.push(path);
            }else{
                paths.push(path);
            }
        }
    }
    paths
}


pub fn delete_path(path: &Path){
    if path.is_file(){
        let _ = remove_file(path);
    }else{
        let _ = remove_dir_all(path);
    }
}

pub fn delete_dir(path: &Path){
    let _ = remove_dir_all(path);
}
