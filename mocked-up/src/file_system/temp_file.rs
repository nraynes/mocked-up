use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use derive_getters::Getters;

use crate::MockError;

#[derive(Getters, Debug)]
pub struct TempFile {
    path: PathBuf,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path) {
            eprintln!(
                "Could not delete file at {}. Error: {}",
                &self
                    .path
                    .to_str()
                    .expect("Could not convert file path to string."),
                e
            )
        }
    }
}

impl TempFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, MockError> {
        let path = path.as_ref();
        if !fs::exists(path)? {
            File::create_new(path)?;
        };
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn write(&mut self, content: &str) -> Result<&mut Self, MockError> {
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        file.write_all(content.as_bytes())?;
        Ok(self)
    }

    pub fn append(&mut self, content: &str) -> Result<&mut Self, MockError> {
        let mut file = File::options().append(true).open(&self.path)?;
        file.write_all(content.as_bytes())?;
        Ok(self)
    }

    pub fn read(&self) -> Result<String, MockError> {
        let mut content = String::new();
        File::open(&self.path)?.read_to_string(&mut content)?;
        Ok(content)
    }
}
