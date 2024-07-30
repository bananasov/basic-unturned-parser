#![feature(str_split_whitespace_remainder)]

use std::path::Path;

pub mod parser;

pub fn get_file_stem(path: &Path) -> Option<String> {
    if let Some(file_stem) = path.file_stem() {
        if let Some(file_stem_str) = file_stem.to_str() {
            return Some(file_stem_str.to_string());
        }
    }

    None
}
