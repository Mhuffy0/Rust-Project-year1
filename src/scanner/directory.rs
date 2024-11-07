use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use super::utils::{human_readable_size, human_readable_time};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: String,
    pub modified: String,
    pub depth: u32,       
    pub is_folder: bool,  
}

#[derive(Debug)]
pub struct FolderStats {
    pub file_name: String,
    pub total_size: String,
    pub allocated_size: String,
    pub item_count: usize,
    pub file_count: usize,
    pub subdir_count: usize,
    pub last_modified: String,
}

pub fn scan_directory(path: &PathBuf) -> Vec<FileEntry> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        if let Ok(entry) = entry {
            let depth = entry.depth() as u32;
            let is_folder = entry.file_type().is_dir();

            let metadata = match fs::metadata(entry.path()) {
                Ok(meta) => meta,
                Err(_) => continue,
            };

            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_size = human_readable_size(metadata.len());
            let modified_time = match metadata.modified() {
                Ok(time) => human_readable_time(time),
                Err(_) => "Unknown".to_string(),
            };

            files.push(FileEntry {
                path: entry.path().to_path_buf(),
                name: file_name,
                size: file_size,
                modified: modified_time,
                depth,          // Assign depth
                is_folder,      // Assign folder status
            });
        }
    }
    files
}



pub fn compute_folder_stats(path: &PathBuf) -> Option<FolderStats> {
    let mut total_size = 0;
    let mut item_count = 0;
    let mut file_count = 0;
    let mut subdir_count = 0;
    let mut last_modified = None;

    for entry in WalkDir::new(path) {
        if let Ok(entry) = entry {
            let metadata = match fs::metadata(entry.path()) {
                Ok(meta) => meta,
                Err(_) => continue,  // Skip if unable to read
            };

            total_size += metadata.len();
            item_count += 1;

            if entry.file_type().is_file() {
                file_count += 1;
            } else if entry.file_type().is_dir() {
                subdir_count += 1;
            }

            if let Ok(modified) = metadata.modified() {
                if last_modified.is_none() || modified > last_modified.unwrap() {
                    last_modified = Some(modified);
                }
            }
        }
    }

    let file_name = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown")
        .to_string();

    Some(FolderStats {
        file_name,
        total_size: human_readable_size(total_size),
        allocated_size: human_readable_size(total_size),
        item_count,
        file_count,
        subdir_count,
        last_modified: match last_modified {
            Some(time) => human_readable_time(time),
            None => "Unknown".to_string(),
        },
    })
}
