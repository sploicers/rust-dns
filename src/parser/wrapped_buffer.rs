use std::result;

pub struct WrappedBuffer {
    pub buf: [u8; 512],
    pos: usize,
}

impl WrappedBuffer {
    pub fn new() -> WrappedBuffer {
        WrappedBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    pub fn read_u8(&mut self) -> result::Result<u8, String> {
        if self.pos > 512 {
            return Err("End of buffer!".into());
        }
        let result: u8 = self.buf[self.pos];
        self.pos += 1;
        Ok(result)
    }

    pub fn read_u16(&mut self) -> result::Result<u16, String> {
        Ok((self.read_u8()? as u16) << 8 | self.read_u8()? as u16)
    }

    pub fn read_u32(&mut self) -> result::Result<u32, String> {
        Ok((self.read_u16()? as u32) << 16 | self.read_u16()? as u32)
    }

    pub fn get_slice(&self, start: usize, len: usize) -> result::Result<&[u8], String> {
        if start + len >= 512 {
            return Err("End of buffer!".into());
        }
        let end = start + len;
        Ok(&self.buf[start..end])
    }

    pub fn advance(&mut self, num_steps: usize) -> result::Result<(), String> {
        self.pos += num_steps;
        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), String> {
        self.pos = pos;
        Ok(())
    }

    pub fn peek(&self, pos: usize) -> Result<u8, String> {
        if pos >= 512 {
            return Err("End of buffer!".into());
        }
        Ok(self.buf[pos])
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}
