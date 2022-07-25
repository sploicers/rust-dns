use super::bitshifting::{get_flag, get_lsb, get_lsn, get_msb};
use super::result_code::ResultCode;
use super::wrapped_buffer::WrappedBuffer;

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16,

    // 1 bit - whether to attempt recursive lookup
    pub recursion_desired: bool,
    // 1 bit - true if message length > 512 bytes
    pub truncated_message: bool,
    // 1 bit - was this an A query?
    pub authoritative_answer: bool,
    // 4 bits - operation, generally set to 0
    pub opcode: u8,
    // 1 bit -is this a request (0/false) or response (1/true)?
    pub response: bool,

    // 4 bits
    pub rescode: ResultCode,
    // 1 bit
    pub checking_disabled: bool,
    // 1 bit
    pub authentic_data: bool,
    // 1 bit - reserved
    pub z: bool,
    // 1 bit - does the server allow/support recursive lookup?
    pub recursion_available: bool,

    // lengths of each section (16 bits each)
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additional: u16,
}

impl DnsHeader {
    // initialize "empty" representation with all default values
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,

            recursion_desired: false,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: false,

            rescode: ResultCode::NOERROR,
            checking_disabled: false,
            authentic_data: false,
            z: false,
            recursion_available: false,

            num_questions: 0,
            num_answers: 0,
            num_authorities: 0,
            num_additional: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        self.read_id(buffer)?;
        self.read_flags(buffer)?;
        self.read_record_counts(buffer)?;
        Ok(())
    }

    pub fn write(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        self.write_id(buffer)?;
        self.write_flags(buffer)?;
        self.write_record_counts(buffer)?;
        Ok(())
    }

    fn read_id(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        self.id = buffer.read_u16()?;
        Ok(())
    }

    fn read_flags(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        let flags = buffer.read_u16()?;
        let most_significant_byte = get_msb(flags);
        let least_significant_byte = get_lsb(flags);

        self.recursion_desired = get_flag(most_significant_byte, 0);
        self.truncated_message = get_flag(most_significant_byte, 1);
        self.authoritative_answer = get_flag(most_significant_byte, 2);
        self.response = get_flag(most_significant_byte, 7);
        self.opcode = get_lsn(most_significant_byte >> 3);

        self.rescode = ResultCode::from_number(least_significant_byte);
        self.checking_disabled = get_flag(least_significant_byte, 4);
        self.authentic_data = get_flag(least_significant_byte, 5);
        self.z = get_flag(least_significant_byte, 6);
        self.recursion_available = get_flag(least_significant_byte, 7);
        Ok(())
    }

    fn read_record_counts(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        self.num_questions = buffer.read_u16()?;
        self.num_answers = buffer.read_u16()?;
        self.num_authorities = buffer.read_u16()?;
        self.num_additional = buffer.read_u16()?;
        Ok(())
    }

    fn write_id(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        Ok(buffer.write_u16(self.id)?)
    }

    fn write_flags(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        let first_flags_byte = self.recursion_desired as u8
            | (self.truncated_message as u8) << 1
            | (self.authoritative_answer as u8) << 2
            | (self.opcode << 3)
            | (self.response as u8) << 7;

        buffer.write_u8(first_flags_byte)?;

        let second_flags_byte = self.rescode as u8
            | ((self.checking_disabled as u8) << 4)
            | ((self.authentic_data as u8) << 5)
            | ((self.z as u8) << 6)
            | ((self.recursion_available as u8) << 7);

        buffer.write_u8(second_flags_byte)?;
        Ok(())
    }

    fn write_record_counts(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        buffer.write_u16(self.num_questions)?;
        buffer.write_u16(self.num_answers)?;
        buffer.write_u16(self.num_authorities)?;
        buffer.write_u16(self.num_additional)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DnsHeader;
    use crate::parser::{
        test_helpers::{get_buffer_at_beginning, GOOGLE_QUERY, RECORD_COUNT_SIZE_BYTES},
        wrapped_buffer::WrappedBuffer,
        ResultCode,
    };
    use std::error::Error;

    const ID_SIZE_BYTES: usize = 2;
    const FLAG_SECTION_SIZE_BYTES: usize = 2;

    #[test]
    fn can_read_id() -> Result<(), Box<dyn Error>> {
        let mut result = DnsHeader::new();
        let expected_id = 48088;
        result.read_id(&mut get_buffer_at_beginning(String::from(GOOGLE_QUERY))?)?;
        assert_eq!(result.id, expected_id);
        Ok(())
    }

    #[test]
    fn can_read_flags() -> Result<(), Box<dyn Error>> {
        let mut result = DnsHeader::new();
        let mut buffer = get_buffer_at_flags_section(String::from(GOOGLE_QUERY))?;
        result.read_flags(&mut buffer)?;

        assert_eq!(result.recursion_desired, true);
        assert_eq!(result.truncated_message, false);
        assert_eq!(result.authoritative_answer, false);
        assert_eq!(result.response, true);
        assert_eq!(result.checking_disabled, false);
        assert_eq!(result.authentic_data, false);
        assert_eq!(result.z, false);
        assert_eq!(result.recursion_available, true);

        Ok(())
    }

    #[test]
    fn can_read_question_count() -> Result<(), Box<dyn Error>> {
        let mut buffer = get_buffer_at_record_count_section(String::from(GOOGLE_QUERY))?;
        let expected_question_count = 1;
        assert_eq!(buffer.read_u16()?, expected_question_count);
        Ok(())
    }

    #[test]
    fn can_read_answer_count() -> Result<(), Box<dyn Error>> {
        let mut buffer = get_buffer_at_record_count_section(String::from(GOOGLE_QUERY))?;
        let expected_answer_count = 1;
        buffer.advance(RECORD_COUNT_SIZE_BYTES)?;
        assert_eq!(buffer.read_u16()?, expected_answer_count);
        Ok(())
    }

    #[test]
    fn can_read_authority_count() -> Result<(), Box<dyn Error>> {
        let mut buffer = get_buffer_at_record_count_section(String::from(GOOGLE_QUERY))?;
        let expected_authority_count = 0;
        buffer.advance(RECORD_COUNT_SIZE_BYTES * 2)?;
        assert_eq!(buffer.read_u16()?, expected_authority_count);
        Ok(())
    }

    #[test]
    fn can_read_additional_record_count() -> Result<(), Box<dyn Error>> {
        let mut buffer = get_buffer_at_record_count_section(String::from(GOOGLE_QUERY))?;
        let expected_additional_record_count = 0;
        buffer.advance(RECORD_COUNT_SIZE_BYTES * 3)?;
        assert_eq!(buffer.read_u16()?, expected_additional_record_count);
        Ok(())
    }

    #[test]
    fn can_write_id() -> Result<(), Box<dyn Error>> {
        let mut buffer = WrappedBuffer::new();
        let mut header_to_write = DnsHeader::new();
        let mut empty_header = DnsHeader::new();

        let expected_id = 8451;
        header_to_write.id = expected_id;

        header_to_write.write(&mut buffer)?;
        buffer.seek(0)?;
        empty_header.read(&mut buffer)?;

        assert_eq!(empty_header.id, expected_id);
        Ok(())
    }

    #[test]
    fn can_write_flags() -> Result<(), Box<dyn Error>> {
        let mut buffer = WrappedBuffer::new();
        let mut header_to_write = DnsHeader::new();
        let mut empty_header = DnsHeader::new();

        header_to_write.recursion_desired = true;
        header_to_write.truncated_message = false;
        header_to_write.authoritative_answer = false;
        header_to_write.opcode = 0;
        header_to_write.response = false;
        header_to_write.rescode = ResultCode::NOERROR;
        header_to_write.checking_disabled = false;
        header_to_write.authentic_data = false;
        header_to_write.z = false;
        header_to_write.recursion_available = false;
        header_to_write.num_questions = 1;
        header_to_write.num_answers = 1;
        header_to_write.num_authorities = 0;
        header_to_write.num_additional = 0;

        header_to_write.write(&mut buffer)?;
        buffer.seek(0)?;
        empty_header.read(&mut buffer)?;

        assert_eq!(
            header_to_write.recursion_desired,
            empty_header.recursion_desired
        );
        assert_eq!(
            header_to_write.truncated_message,
            empty_header.truncated_message
        );
        assert_eq!(
            header_to_write.authoritative_answer,
            empty_header.authoritative_answer
        );
        assert_eq!(header_to_write.opcode, empty_header.opcode);
        assert_eq!(header_to_write.response, empty_header.response);
        assert_eq!(header_to_write.rescode, empty_header.rescode);
        assert_eq!(
            header_to_write.checking_disabled,
            empty_header.checking_disabled
        );
        assert_eq!(header_to_write.authentic_data, empty_header.authentic_data);
        assert_eq!(header_to_write.z, empty_header.z);
        assert_eq!(
            header_to_write.recursion_available,
            empty_header.recursion_available
        );

        Ok(())
    }

    #[test]
    fn can_write_record_counts() -> Result<(), Box<dyn Error>> {
        let mut buffer = WrappedBuffer::new();
        let mut header_to_write = DnsHeader::new();
        let mut empty_header = DnsHeader::new();

        header_to_write.num_questions = 1;
        header_to_write.num_answers = 1;
        header_to_write.num_authorities = 0;
        header_to_write.num_additional = 0;

        header_to_write.write(&mut buffer)?;
        buffer.seek(0)?;
        empty_header.read(&mut buffer)?;

        assert_eq!(header_to_write.num_questions, empty_header.num_questions);
        assert_eq!(header_to_write.num_answers, empty_header.num_answers);
        assert_eq!(
            header_to_write.num_authorities,
            empty_header.num_authorities
        );
        assert_eq!(header_to_write.num_additional, empty_header.num_additional);
        Ok(())
    }

    fn get_buffer_at_flags_section(input_file: String) -> Result<WrappedBuffer, Box<dyn Error>> {
        let mut buffer = get_buffer_at_beginning(input_file)?;
        buffer.advance(ID_SIZE_BYTES)?;
        Ok(buffer)
    }

    fn get_buffer_at_record_count_section(
        input_file: String,
    ) -> Result<WrappedBuffer, Box<dyn Error>> {
        let mut buffer = get_buffer_at_beginning(input_file)?;
        buffer.advance(ID_SIZE_BYTES + FLAG_SECTION_SIZE_BYTES)?;
        Ok(buffer)
    }
}
