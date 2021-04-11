#[cfg(test)]
mod rename_test {

    use std::fs;
    use std::io;
    use std::path;

    #[test]
    fn source_missing(){
        let temp = path::Path::new(".test").join("rename_test").join("source_missing");
        fs::create_dir_all(&temp).unwrap();
        let src = temp.join("not-exist");
        let dst = temp.join("whatever");
        fs::remove_file(&src).ok();

        let r = rename_file::rename(src, dst);
        assert_eq!(false, r.is_ok());
        assert_eq!(true,  r.is_err());

        let e = r.err().unwrap();
        match e {
            rename_file::RenameError::NotFound(ref e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {},
                    _                       => panic!("Unexpected error"),
                }
            },
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn target_directory(){
        let temp = path::Path::new(".test").join("rename_test").join("target_directory");
        fs::create_dir_all(&temp).unwrap();
        let src = temp.join("file-exist");
        let dst = temp.join("directory");
        fs::File::create(&src).unwrap();
        fs::create_dir_all(&dst).unwrap();

        let r = rename_file::rename(src, dst);
        assert_eq!(false, r.is_ok());
        assert_eq!(true,  r.is_err());

        let e = r.err().unwrap();
        match e {
            rename_file::RenameError::PermissionDenied(ref e) => {
                match e.kind() {
                    io::ErrorKind::PermissionDenied => {},
                    _                               => panic!("Unexpected error"),
                }
            },
            rename_file::RenameError::IsDirectory(ref e) => {},
            _ => panic!("Unexpected error: {}", e),
        }
    }
}
