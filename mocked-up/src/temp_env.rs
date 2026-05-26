use std::path::PathBuf;

use derive_getters::Getters;
use rand::RngExt;

use crate::{MockError, TempDir};

#[derive(Getters)]
pub struct TempEnv {
    root: TempDir,
}

impl TempEnv {
    pub fn new() -> Result<Self, MockError> {
        let temp_dir = std::env::temp_dir();
        let env_root = temp_dir.join(PathBuf::from(Self::generate_name()));
        let root = TempDir::new(&env_root)?;
        Ok(Self { root })
    }

    pub fn env(&mut self) -> &mut TempDir {
        &mut self.root
    }

    fn generate_name() -> String {
        let mut rng = rand::rng();
        let name_len: u32 = rng.random_range(8..=14);
        let mut name = String::new();
        for _ in 0..name_len {
            name.push(rng.sample(rand::distr::Alphanumeric) as char);
        }
        name
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn test_generate_name() {
        for _ in 0..50 {
            let name = TempEnv::generate_name();
            assert!(8 <= name.len());
            assert!(name.len() <= 14);
        }
    }

    #[test]
    fn test_create_temp_env() {
        let env_path;
        {
            let temp_env = TempEnv::new().unwrap();
            env_path = temp_env.root().path().clone();
            assert!(fs::exists(&env_path).unwrap());
        }
        assert!(!fs::exists(&env_path).unwrap());
    }
}
