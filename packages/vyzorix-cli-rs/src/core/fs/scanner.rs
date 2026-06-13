use ignore::WalkBuilder;
use std::path::PathBuf;

pub fn calculate_workspace_files(dir: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    // Advanced builder enforcing .gitignore and nested overrides
    for result in WalkBuilder::new(dir).hidden(false).build() {
        if let Ok(entry) = result {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                files.push(entry.into_path());
            }
        }
    }
    files
}
