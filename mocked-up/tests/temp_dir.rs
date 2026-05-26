#[cfg(test)]
mod test {
    use std::fs;

    use mocked_up::TempEnv;

    #[test]
    fn dir_ops() {
        let mut temp = TempEnv::new().unwrap();
        temp.env().mkdir("test_dir").unwrap();
        let test_dir_path = temp
            .env()
            .dir("test_dir")
            .ok_or("test_dir was not found in the temp environment!")
            .unwrap()
            .path()
            .clone();

        assert!(fs::exists(&test_dir_path).unwrap());

        temp.env().rmdir("test_dir");

        assert!(!fs::exists(test_dir_path).unwrap());
        assert!(temp.env().dir("test_dir").is_none());
    }

    #[test]
    fn file_ops() {
        let mut temp = TempEnv::new().unwrap();
        temp.env().touch("test_file").unwrap();
        let test_file_path = temp
            .env()
            .file("test_file")
            .ok_or("test_dir was not found in the temp environment!")
            .unwrap()
            .path()
            .clone();

        assert!(fs::exists(&test_file_path).unwrap());

        temp.env().rm("test_file");

        assert!(!fs::exists(test_file_path).unwrap());
        assert!(temp.env().file("test_file").is_none());
    }
}
