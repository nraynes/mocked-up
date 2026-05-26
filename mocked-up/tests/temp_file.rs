#[cfg(test)]
mod test {
    use mocked_up::TempEnv;

    #[test]
    fn ops() {
        let mut temp = TempEnv::new().unwrap();
        temp.env().touch("test_file").unwrap();
        let test_file = temp
            .env()
            .file("test_file")
            .ok_or("test_dir was not found in the temp environment!")
            .unwrap();

        let contents = test_file.read().unwrap();
        assert_eq!(&contents, "");

        test_file
            .write("Some content was written to the file.")
            .unwrap();

        let contents = test_file.read().unwrap();
        assert_eq!(&contents, "Some content was written to the file.");

        test_file
            .append("This line was appended to the file.")
            .unwrap();

        let contents = test_file.read().unwrap();
        assert_eq!(
            &contents,
            "Some content was written to the file.This line was appended to the file."
        );

        test_file
            .write("This should overwrite the files current contents.")
            .unwrap();

        let contents = test_file.read().unwrap();
        assert_eq!(
            &contents,
            "This should overwrite the files current contents."
        );
    }
}
