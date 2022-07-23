use std::{error::Error, fs::File};

const TEST_DATA_DIR: &str = "test_data";

pub fn open_test_file(filename: String) -> std::io::Result<File> {
    File::open(format!(
        "{}/{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        TEST_DATA_DIR,
        filename
    ))
}

pub fn expect_error<T>(result: Result<T, String>, msg: &str) -> Result<(), Box<dyn Error>> {
    match result {
        Err(_) => Ok(()),
        _ => panic!("{}", msg),
    }
}
