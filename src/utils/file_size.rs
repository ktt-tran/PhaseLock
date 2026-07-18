use std::fs;
use std::path::Path;

pub const KB: f64 = 1024.0;
pub const MB: f64 = KB * 1024.0;
pub const GB: f64 = MB * 1024.0;

pub fn total_size(path: &Path) -> u64 {
    if path.is_file() {
        fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(0)
    } else if path.is_dir() {
        let mut size = 0;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                size += total_size(&entry.path());
            }
        }

        size
    } else {
        0
    }
}

pub fn format_size(bytes: u64) -> String {

    let bytes = bytes as f64;

    if bytes >= GB {
        format!("{:.2} GB", bytes / GB)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes / KB)
    } else {
        format!("{:.0} B", bytes)
    }
}