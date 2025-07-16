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
            mark.handle_dir(path.read_dir().unwrap());
        } else if path.is_file() {
            mark.handle_file(path);
        }

        mark.content
    }

    fn handle_dir(&mut self, dir: ReadDir) {
        for entry in dir {
            let entry = entry.unwrap();

            let file_type = entry.file_type().unwrap();

            if file_type.is_dir() {
                self.handle_dir(entry.path().as_path().read_dir().unwrap());
            } else if file_type.is_file() {
                self.handle_file(entry.path().as_path());
            }
        }
    }

    fn handle_file(&mut self, path: &Path) {
        let content = fs::read_to_string(path).unwrap();

        let ext = Language::from_ext(path.extension().unwrap());

        if let Some(ext) = ext {
            self.content.push_str(&format!(
                "{}\n```{ext}\n{content}\n```\n\n",
                path.to_str().unwrap()
            ));
        }
    }
}
