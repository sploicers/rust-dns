use super::bitshifting::{get_lsb, get_msb, get_nth_octal};

const BUFFER_SIZE: usize = 512;

pub struct WrappedBuffer {
    raw_buffer: [u8; BUFFER_SIZE],
    position: usize,
}

impl WrappedBuffer {
    pub fn new() -> WrappedBuffer {
        WrappedBuffer {
            raw_buffer: [0; BUFFER_SIZE],
            position: 0,
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, String> {
        if self.position >= BUFFER_SIZE {
            return Err("End of buffer!".into());
        }
        let result: u8 = self.raw_buffer[self.position];
        self.advance(1)?;
        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        Ok((self.read_u8()? as u16) << 8 | self.read_u8()? as u16)
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        Ok((self.read_u16()? as u32) << 16 | self.read_u16()? as u32)
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), String> {
        if self.position >= BUFFER_SIZE {
            return Err("End of buffer!".into());
        }
        self.raw_buffer[self.position] = value;
        self.advance(1)?;
        Ok(())
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), String> {
        self.write_u8(get_msb(value))?;
        self.write_u8(get_lsb(value))?;
        Ok(())
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), String> {
        for i in 1..=4 {
            self.write_u8(get_nth_octal(value, i))?;
        }
        Ok(())
    }

    pub fn get_slice(&self, start: usize, len: usize) -> Result<&[u8], String> {
        if start + len >= BUFFER_SIZE {
            return Err("End of buffer!".into());
        }
        let end = start + len;
        Ok(&self.raw_buffer[start..end])
    }

    pub fn as_slice(&mut self) -> Result<&mut [u8], String> {
        Ok(&mut self.raw_buffer[0..BUFFER_SIZE])
    }

    pub fn advance(&mut self, num_steps: usize) -> Result<(), String> {
        self.position += num_steps;
        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), String> {
        self.position = pos;
        Ok(())
    }

    pub fn peek(&self, pos: usize) -> Result<u8, String> {
        if pos >= BUFFER_SIZE {
            return Err("End of buffer!".into());
        }
        Ok(self.raw_buffer[pos])
    }

    pub fn pos(&self) -> usize {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::{WrappedBuffer, BUFFER_SIZE};
    use crate::parser::test_helpers::expect_error;
    use std::error::Error;

    const BUFFER_OVERRUN_MESSAGE: &str = "Expected buffer overrun.";

    #[test]
    fn reading_fails_on_buffer_overrun() -> Result<(), Box<dyn Error>> {
        let mut buffer = WrappedBuffer::new();
        buffer.seek(BUFFER_SIZE)?;

        expect_error(buffer.read_u8(), BUFFER_OVERRUN_MESSAGE)?;
        expect_error(buffer.read_u16(), BUFFER_OVERRUN_MESSAGE)?;
        expect_error(buffer.read_u32(), BUFFER_OVERRUN_MESSAGE)?;
        Ok(())
    }

    #[test]
    fn peek_fails_on_buffer_overrun() -> Result<(), Box<dyn Error>> {
        let buffer = WrappedBuffer::new();
        expect_error(buffer.peek(BUFFER_SIZE), BUFFER_OVERRUN_MESSAGE)?;
        Ok(())
    }

    #[test]
    fn get_slice_fails_on_buffer_overrun() -> Result<(), Box<dyn Error>> {
        let buffer = WrappedBuffer::new();
        expect_error(
            buffer.get_slice(BUFFER_SIZE / 2, BUFFER_SIZE / 2 + 1),
            BUFFER_OVERRUN_MESSAGE,
        )?;
        Ok(())
    }
}
