use owo_colors::OwoColorize;
use remove_dir_all::remove_dir_all;
use walkdir::WalkDir;

use crate::data::package::PackageConfig;

pub fn execute_clear_operation() {
    let package = PackageConfig::read();
    for entry in WalkDir::new(package.dependencies_dir.canonicalize().unwrap()).min_depth(1) {
        let path = entry.unwrap().into_path();
        if path.is_symlink() {
            if path.is_dir() {
                if let Err(e) = symlink::remove_symlink_dir(&path) {
                    println!(
                        "Failed to remove symlink for directory {}: {}",
                        path.display().bright_yellow(),
                        e
                    );
                }
            } else if path.is_file() {
                if let Err(e) = symlink::remove_symlink_file(&path) {
                    println!(
                        "Failed to remove symlink for directory {}: {}",
                        path.display().bright_yellow(),
                        e
                    );
                }
            }
        };
    }

    remove_dir_all(package.dependencies_dir.canonicalize().unwrap())
        .expect("Failed to remove cached folders");
    std::fs::remove_file("qpm.shared.json").ok();
}
