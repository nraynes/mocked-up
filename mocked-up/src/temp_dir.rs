use std::{collections::HashMap, fs, path::PathBuf};

use derive_getters::Getters;

use crate::{MockError, TempFile};

#[derive(Getters)]
pub struct TempDir {
    path: PathBuf,
    files: HashMap<String, TempFile>,
    dirs: HashMap<String, Self>,
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.path) {
            eprintln!(
                "Could not delete directory at {}. Error: {}",
                &self
                    .path
                    .to_str()
                    .expect("Could not convert directory path to string."),
                e
            )
        }
    }
}

impl TempDir {
    pub fn new(path: &PathBuf) -> Result<Self, MockError> {
        if !fs::exists(path)? {
            fs::create_dir_all(path)?;
        };
        Ok(Self {
            path: path.clone(),
            files: HashMap::new(),
            dirs: HashMap::new(),
        })
    }

    pub fn touch(&mut self, name: &str) -> Result<&mut Self, MockError> {
        self.files.insert(
            name.to_string(),
            TempFile::new(&self.path.join(PathBuf::from(name)))?,
        );
        Ok(self)
    }

    pub fn mkdir(&mut self, name: &str) -> Result<&mut Self, MockError> {
        self.dirs.insert(
            name.to_string(),
            Self::new(&self.path.join(PathBuf::from(name)))?,
        );
        Ok(self)
    }

    pub fn rm(&mut self, name: &str) -> &mut Self {
        if self.files.remove(name).is_none() {
            match self.path.join(PathBuf::from(name)).to_str() {
                Some(file_path) => eprintln!("Could not remove file at {}, not found.", file_path),
                None => eprintln!("Could not remove file, not found."),
            }
        }
        self
    }

    pub fn rmdir(&mut self, name: &str) -> &mut Self {
        if self.dirs.remove(name).is_none() {
            match self.path.join(PathBuf::from(name)).to_str() {
                Some(file_path) => {
                    eprintln!("Could not remove directory at {}, not found.", file_path)
                }
                None => eprintln!("Could not remove directory, not found."),
            }
        }
        self
    }

    pub fn dir(&mut self, name: &str) -> Option<&mut Self> {
        self.dirs.get_mut(name)
    }

    pub fn file(&mut self, name: &str) -> Option<&mut TempFile> {
        self.files.get_mut(name)
    }
}
