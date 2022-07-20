use std::io::Result;

use super::parsing_error::ParseError;

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

    pub(crate) fn read_query_name(&mut self, result: &mut String) -> Result<()> {
        let mut local_pos = self.pos();
        let mut delimiter = "";
        let mut have_jumped = false;
        let mut num_jumps = 0;
        let max_jumps = 5; // This is so we can bail out of any malicious packets designed to send the parser into an infinite loop.

        loop {
            if num_jumps > max_jumps {
                return ParseError::new(
                    format!(
                        "Too many jumps performed while parsing DNS packet ({}) - could be a malicious packet containing a cycle.",
                        num_jumps
                    )
                );
            }
            let label_length_byte: u8 = self.peek(local_pos)?;
            // If the two most significant bits of the label length are set, this represents a jump to a different position.
            let should_jump = label_length_byte & 0xC0 == 0xC0;

            if should_jump {
                if !have_jumped {
                    // Since we're jumping, we want to move past the two length bytes.
                    self.seek(local_pos + 2)?;
                }
                let next_byte: u8 = self.peek(local_pos + 1)?;

                // Compute the jump destination. The jump destination is obtained by:
                // 1) unsetting the most significant two bits of the length byte
                // 2) combining this with the next byte and treating the two together as a new 16 bit number
                //    representing the position in the buffer from which parsing should proceed.
                let jump_destination_pos =
                    ((((label_length_byte as u16) ^ 0xC0) << 8) | (next_byte as u16)) as usize;

                local_pos = jump_destination_pos;
                have_jumped = true;
                num_jumps += 1;
                continue;
            } else {
                local_pos += 1;

                if label_length_byte == 0 {
                    break; // The query name section is null-terminated.
                }
                result.push_str(delimiter);
                delimiter = ".";

                let char_count = label_length_byte as usize;
                let slice = self.get_slice(local_pos, char_count)?;
                result.push_str(&String::from_utf8_lossy(slice).to_lowercase());
                local_pos += char_count;
            }
        }

        if !have_jumped {
            self.seek(local_pos)?;
        }
        Ok(())
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        if self.pos > 512 {
            return ParseError::new("End of buffer!".into());
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

    fn get_slice(&self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= 512 {
            return ParseError::new("End of buffer!".into());
        }
        let end = start + len;
        Ok(&self.buf[start..end])
    }

    pub fn advance(&mut self, num_steps: usize) -> Result<()> {
        self.pos += num_steps;
        Ok(())
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    fn peek(&self, pos: usize) -> Result<u8> {
        if pos >= 512 {
            return ParseError::new("End of buffer!".into());
        }
        Ok(self.buf[pos])
    }
}
