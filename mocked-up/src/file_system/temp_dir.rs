use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use derive_getters::Getters;

use crate::{MockError, file_system::TempFile};

#[derive(Getters, Debug)]
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
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, MockError> {
        let path = path.as_ref();
        if !fs::exists(path)? {
            fs::create_dir_all(path)?;
        };
        Ok(Self {
            path: path.to_path_buf(),
            files: HashMap::new(),
            dirs: HashMap::new(),
        })
    }

    pub fn touch(&mut self, name: &str) -> Result<&mut Self, MockError> {
        self.files
            .insert(name.to_string(), TempFile::new(&self.path.join(name))?);
        Ok(self)
    }

    pub fn mkdir(&mut self, name: &str) -> Result<&mut Self, MockError> {
        self.dirs
            .insert(name.to_string(), Self::new(&self.path.join(name))?);
        Ok(self)
    }

    pub fn touch_and<F: FnMut(&mut TempFile)>(
        &mut self,
        name: &str,
        mut f: F,
    ) -> Result<&mut Self, MockError> {
        let mut new_file = TempFile::new(&self.path.join(name))?;
        f(&mut new_file);
        self.files.insert(name.to_string(), new_file);
        Ok(self)
    }

    pub fn mkdir_and<F: FnMut(&mut Self)>(
        &mut self,
        name: &str,
        mut f: F,
    ) -> Result<&mut Self, MockError> {
        let mut new_dir = Self::new(&self.path.join(name))?;
        f(&mut new_dir);
        self.dirs.insert(name.to_string(), new_dir);
        Ok(self)
    }

    pub fn rm(&mut self, name: &str) -> &mut Self {
        if self.files.remove(name).is_none() {
            match self.path.join(name).to_str() {
                Some(file_path) => eprintln!("Could not remove file at {}, not found.", file_path),
                None => eprintln!("Could not remove file, not found."),
            }
        }
        self
    }

    pub fn rmdir(&mut self, name: &str) -> &mut Self {
        if self.dirs.remove(name).is_none() {
            match self.path.join(name).to_str() {
                Some(file_path) => {
                    eprintln!("Could not remove directory at {}, not found.", file_path)
                }
                None => eprintln!("Could not remove directory, not found."),
            }
        }
        self
    }

    fn refresh(&mut self) -> Result<(), MockError> {
        for item in fs::read_dir(&self.path)? {
            let item = item?;
            let item_file_name = item.file_name();
            let item_name = item_file_name
                .to_str()
                .ok_or("Could not convert OS string to string.")?;
            if item.file_type()?.is_file() {
                if !self.files.contains_key(item_name) {
                    self.files
                        .insert(item_name.to_string(), TempFile::new(&item.path())?);
                }
            } else if item.file_type()?.is_dir() {
                if !self.dirs.contains_key(item_name) {
                    self.dirs
                        .insert(item_name.to_string(), Self::new(&item.path())?);
                }
            }
        }
        Ok(())
    }

    pub fn dir(&mut self, name: &str) -> Option<&mut Self> {
        self.refresh().ok();
        self.dirs.get_mut(name)
    }

    pub fn file(&mut self, name: &str) -> Option<&mut TempFile> {
        self.refresh().ok();
        self.files.get_mut(name)
    }
}
