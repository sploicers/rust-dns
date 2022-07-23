pub struct WrappedBuffer {
    pub raw_buffer: [u8; 512],
    position: usize,
}

impl WrappedBuffer {
    pub fn new() -> WrappedBuffer {
        WrappedBuffer {
            raw_buffer: [0; 512],
            position: 0,
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, String> {
        if self.position > 512 {
            return Err("End of buffer!".into());
        }
        let result: u8 = self.raw_buffer[self.position];
        self.position += 1;
        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        Ok((self.read_u8()? as u16) << 8 | self.read_u8()? as u16)
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        Ok((self.read_u16()? as u32) << 16 | self.read_u16()? as u32)
    }

    pub fn get_slice(&self, start: usize, len: usize) -> Result<&[u8], String> {
        if start + len >= 512 {
            return Err("End of buffer!".into());
        }
        let end = start + len;
        Ok(&self.raw_buffer[start..end])
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
        if pos >= 512 {
            return Err("End of buffer!".into());
        }
        Ok(self.raw_buffer[pos])
    }

    pub fn pos(&self) -> usize {
        self.position
    }
}
