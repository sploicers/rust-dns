use super::parser_result::Result;

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

    pub fn read_u8(&mut self) -> Result<u8> {
        if self.pos > 512 {
            return Err("End of buffer!".into());
        }
        let result: u8 = self.buf[self.pos];
        self.pos += 1;
        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok((self.read_u8()? as u16) << 8 | self.read_u8()? as u16)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok((self.read_u16()? as u32) << 16 | self.read_u16()? as u32)
    }

    pub fn get_slice(&self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= 512 {
            return Err("End of buffer!".into());
        }
        let end = start + len;
        Ok(&self.buf[start..end])
    }

    pub fn advance(&mut self, num_steps: usize) -> Result<()> {
        self.pos += num_steps;
        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    pub fn peek(&self, pos: usize) -> Result<u8> {
        if pos >= 512 {
            return Err("End of buffer!".into());
        }
        Ok(self.buf[pos])
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}
