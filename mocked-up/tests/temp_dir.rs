#[cfg(test)]
mod test {
    use std::fs;

    use mocked_up::file_system::TempEnv;

    #[test]
    fn dir_builder() {
        let mut temp = TempEnv::new().unwrap();
        temp.env()
            .mkdir_and("test_dir_one", |d| {
                d.mkdir("inner_one_test_dir_one")
                    .unwrap()
                    .mkdir("inner_one_test_dir_two")
                    .unwrap()
                    .touch("inner_one_test_file_one")
                    .unwrap();
            })
            .unwrap()
            .mkdir_and("test_dir_two", |d| {
                d.mkdir("inner_two_test_dir_one")
                    .unwrap()
                    .mkdir("inner_two_test_dir_two")
                    .unwrap()
                    .touch("inner_two_test_file_one")
                    .unwrap();
            })
            .unwrap()
            .touch("file_one")
            .unwrap()
            .touch_and("file_two", |f| {
                f.write("Test content one two three.").unwrap();
            })
            .unwrap();

        assert!(fs::exists(temp.env().path()).unwrap());

        assert!(fs::exists(temp.env().dir("test_dir_one").unwrap().path()).unwrap());
        assert!(
            fs::exists(
                temp.env()
                    .dir("test_dir_one")
                    .unwrap()
                    .dir("inner_one_test_dir_one")
                    .unwrap()
                    .path()
            )
            .unwrap()
        );
        assert!(
            fs::exists(
                temp.env()
                    .dir("test_dir_one")
                    .unwrap()
                    .dir("inner_one_test_dir_two")
                    .unwrap()
                    .path()
            )
            .unwrap()
        );
        assert!(
            fs::exists(
                temp.env()
                    .dir("test_dir_one")
                    .unwrap()
                    .file("inner_one_test_file_one")
                    .unwrap()
                    .path()
            )
            .unwrap()
        );

        assert!(fs::exists(temp.env().dir("test_dir_two").unwrap().path()).unwrap());
        assert!(
            fs::exists(
                temp.env()
                    .dir("test_dir_two")
                    .unwrap()
                    .dir("inner_two_test_dir_one")
                    .unwrap()
                    .path()
            )
            .unwrap()
        );
        assert!(
            fs::exists(
                temp.env()
                    .dir("test_dir_two")
                    .unwrap()
                    .dir("inner_two_test_dir_two")
                    .unwrap()
                    .path()
            )
            .unwrap()
        );
        assert!(
            fs::exists(
                temp.env()
                    .dir("test_dir_two")
                    .unwrap()
                    .file("inner_two_test_file_one")
                    .unwrap()
                    .path()
            )
            .unwrap()
        );

        assert!(fs::exists(temp.env().file("file_one").unwrap().path()).unwrap());
        assert!(fs::exists(temp.env().file("file_two").unwrap().path()).unwrap());

        assert_eq!(
            fs::read_to_string(temp.env().file("file_two").unwrap().path())
                .unwrap()
                .as_str(),
            "Test content one two three."
        );
    }

    #[test]
    fn outside_dir_ops() {
        let mut temp = TempEnv::new().unwrap();
        fs::create_dir(temp.env().path().join("test_dir")).unwrap();
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
    fn outside_file_ops() {
        let mut temp = TempEnv::new().unwrap();
        fs::File::create(temp.env().path().join("test_file")).unwrap();
        let test_file_path = temp
            .env()
            .file("test_file")
            .ok_or("test_file was not found in the temp environment!")
            .unwrap()
            .path()
            .clone();

        assert!(fs::exists(&test_file_path).unwrap());

        temp.env().rm("test_file");

        assert!(!fs::exists(test_file_path).unwrap());
        assert!(temp.env().file("test_file").is_none());
    }

    #[test]
    fn file_ops() {
        let mut temp = TempEnv::new().unwrap();
        temp.env().touch("test_file").unwrap();
        let test_file_path = temp
            .env()
            .file("test_file")
            .ok_or("test_file was not found in the temp environment!")
            .unwrap()
            .path()
            .clone();

        assert!(fs::exists(&test_file_path).unwrap());

        temp.env().rm("test_file");

        assert!(!fs::exists(test_file_path).unwrap());
        assert!(temp.env().file("test_file").is_none());
    }
}
