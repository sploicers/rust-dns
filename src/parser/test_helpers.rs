#![allow(unused)]
use super::wrapped_buffer::WrappedBuffer;
use std::{error::Error, fs::File, io::Read};

const TEST_DATA_DIR: &str = "test_data";
pub const GOOGLE_QUERY: &str = "google_query_response.txt";
pub const HEADER_LENGTH_BYTES: usize = 12;
pub const RECORD_COUNT_SIZE_BYTES: usize = 2;

pub fn open_test_file(filename: String) -> std::io::Result<File> {
    File::open(format!(
        "{}/{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        TEST_DATA_DIR,
        filename
    ))
}

pub fn get_buffer_at_question_section(input_file: String) -> Result<WrappedBuffer, Box<dyn Error>> {
    let mut buffer = get_buffer_at_beginning(input_file)?;
    buffer.advance(HEADER_LENGTH_BYTES)?; // Advance the buffer past the header to the beginning of the question section.
    Ok(buffer)
}

pub fn get_buffer_at_beginning(input_file: String) -> Result<WrappedBuffer, Box<dyn Error>> {
    let mut buffer = WrappedBuffer::new();
    let mut file = open_test_file(input_file)?;
    file.read(&mut buffer.as_slice()?)?;
    Ok(buffer)
}

pub fn expect_error<T>(result: Result<T, String>, msg: &str) -> Result<(), Box<dyn Error>> {
    match result {
        Err(_) => Ok(()),
        _ => panic!("{}", msg),
    }
}

pub fn are_same_enum_variant<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
