use super::wrapped_buffer::WrappedBuffer;

const MAX_JUMPS: usize = 5;

pub struct QueryName {
    pub value: String,
    pub num_jumps: usize,
    have_jumped: bool,
    delimiter: String,
}

impl QueryName {
    pub fn new() -> QueryName {
        QueryName {
            value: String::new(),
            num_jumps: 0,
            have_jumped: false,
            delimiter: String::from(""),
        }
    }

    pub fn read(&mut self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        if self.num_jumps > MAX_JUMPS {
            return Err(
                format!(
                    "Too many jumps performed while parsing DNS packet ({}) - could be a malicious packet containing a cycle.",
                    self.num_jumps
                )
            );
        }

        let local_position = buffer.pos();
        let label_length_byte: u8 = buffer.peek()?;
        // If the two most significant bits of the label length are set, this represents a jump to a different position instead.
        let should_jump = label_length_byte & 0xC0 == 0xC0;

        if should_jump {
            if !self.have_jumped {
                buffer.advance(2)?; // We're jumping, so move past the last two bytes, as they represented the jump destination.
            }

            // Compute the jump destination. The jump destination is obtained by:
            // 1) unsetting the most significant two bits of the length byte
            // 2) combining this with the next byte and treating the two together as a new 16 bit number
            //    representing the position in the buffer from which parsing should proceed.
            let next_byte: u8 = buffer.get(local_position + 1)?;
            let jump_destination =
                ((((label_length_byte as u16) ^ 0xC0) << 8) | (next_byte as u16)) as usize;

            // Jump to the specified position, read the name, then jump back
            buffer.seek(jump_destination)?;
            self.have_jumped = true;
            self.num_jumps = 1;

            self.read(buffer)?;
            buffer.seek(local_position)?;
        } else {
            if label_length_byte == 0 {
                return Ok(());
            }

            // Move past length byte, we're at the domain name now.
            buffer.advance(1)?;
            self.value.push_str(&self.delimiter);

            let name = buffer.get_slice(buffer.pos(), label_length_byte as usize)?;
            self.value
                .push_str(&String::from_utf8_lossy(name).to_lowercase());

            self.delimiter = String::from(".");
            buffer.advance(label_length_byte as usize)?;
            self.read(buffer)?;
        }

        Ok(())
    }
}

#[cfg(tests)]
mod tests {
    use super::{QueryName, QueryNameParser};
    use crate::parser::test_helpers::{get_buffer_at_question_section, GOOGLE_QUERY};
    use std::error::Error;

    #[test]
    fn reads_domain_name_successfully() -> Result<(), Box<dyn Error>> {
        let mut domain_name = String::new();
        let expected_domain_name = String::from("google.com");
        QueryName::read(
            &mut get_buffer_at_question_section(String::from(GOOGLE_QUERY))?,
            &mut domain_name,
        )?;
        assert_eq!(domain_name, expected_domain_name);
        Ok(())
    }

    #[test]
    #[ignore = "Need to create a packet exhibiting this scenario in a hex editor or something."]
    fn parsing_fails_for_packet_with_too_many_jumps() -> Result<(), Box<dyn Error>> {
        todo!();
    }
}
