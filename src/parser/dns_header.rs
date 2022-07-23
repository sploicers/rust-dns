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
        self.id = buffer.read_u16()?;
        self.read_flags(buffer)?;
        self.read_record_counts(buffer)?;
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
}
