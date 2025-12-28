#![deny(clippy::unwrap_used)]

use std::{
    fs::{self, ReadDir},
    path::Path,
};

use crate::file_extensions::Language;

mod file_extensions;

pub struct MarkItDown {
    content: String,
}

impl MarkItDown {
    pub fn get_content(path: &str) -> String {
        let mut mark = MarkItDown {
            content: String::new(),
        };

        let path = Path::new(path);

        if path.is_dir() {
            if let Ok(dir) = path.read_dir() {
                mark.handle_dir(dir);
            }
        } else if path.is_file() {
            mark.handle_file(path);
        }

        mark.content
    }

    fn handle_dir(&mut self, dir: ReadDir) {
        for entry in dir {
            let Ok(entry) = entry else {
                continue;
            };

            let Ok(file_type) = entry.file_type() else {
                continue;
            };

            if file_type.is_dir() {
                if let Ok(sub_dir) = entry.path().as_path().read_dir() {
                    self.handle_dir(sub_dir);
                }
            } else if file_type.is_file() {
                self.handle_file(entry.path().as_path());
            }
        }
    }

    fn handle_file(&mut self, path: &Path) {
        let Ok(content) = fs::read_to_string(path) else {
            return;
        };

        let Some(extension) = path.extension() else {
            return;
        };

        let ext = Language::from_ext(extension);

        if let Some(ext) = ext
            && let Some(path_str) = path.to_str()
        {
            self.content
                .push_str(&format!("{}\n```{ext}\n{content}\n```\n\n", path_str));
        }
    }
}
