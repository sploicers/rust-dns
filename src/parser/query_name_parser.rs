use super::wrapped_buffer::WrappedBuffer;

pub struct QueryName {}

pub trait QueryNameParser {
    fn read(buffer: &mut WrappedBuffer, result: &mut String) -> Result<(), String> {
        let mut local_pos = buffer.pos();
        let mut delimiter = "";
        let mut have_jumped = false;
        let mut num_jumps = 0;
        let max_jumps = 5; // This is so we can bail out of any malicious packets designed to send the parser into an infinite loop.

        loop {
            if num_jumps > max_jumps {
                return Err(
                    format!(
                        "Too many jumps performed while parsing DNS packet ({}) - could be a malicious packet containing a cycle.",
                        num_jumps
                    )
                );
            }
            let label_length_byte: u8 = buffer.peek(local_pos)?;
            // If the two most significant bits of the label length are set, this represents a jump to a different position.
            let should_jump = label_length_byte & 0xC0 == 0xC0;

            if should_jump {
                if !have_jumped {
                    // Since we're jumping, we want to move past the two length bytes.
                    buffer.seek(local_pos + 2)?;
                }
                let next_byte: u8 = buffer.peek(local_pos + 1)?;

                // Compute the jump destination. The jump destination is obtained by:
                // 1) unsetting the most significant two bits of the length byte
                // 2) combining this with the next byte and treating the two together as a new 16 bit number
                //    representing the position in the buffer from which parsing should proceed.
                let jump_destination_pos =
                    ((((label_length_byte as u16) ^ 0xC0) << 8) | (next_byte as u16)) as usize;

                local_pos = jump_destination_pos;
                have_jumped = true;
                num_jumps += 1;
            } else {
                local_pos += 1;

                if label_length_byte == 0 {
                    break; // The query name section is null-terminated.
                }
                result.push_str(delimiter);
                delimiter = ".";

                let char_count = label_length_byte as usize;
                let slice = buffer.get_slice(local_pos, char_count)?;
                result.push_str(&String::from_utf8_lossy(slice).to_lowercase());
                local_pos += char_count;
            }
        }

        if !have_jumped {
            buffer.seek(local_pos)?;
        }
        Ok(())
    }

    fn write(buffer: &mut WrappedBuffer, name: &str) -> Result<(), String> {
        for segment in name.split('.') {
            let length = segment.len() as u8;

            if length > 0x3f {
                return Err("Individual domain name segments cannot exceed 63 chars long".into());
            }

            buffer.write_u8(length)?;

            for byte in segment.as_bytes() {
                buffer.write_u8(*byte)?;
            }
        }
        buffer.write_u8(0)?; // Null-terminate the name.
        Ok(())
    }
}

impl QueryNameParser for QueryName {}

#[cfg(test)]
mod tests {
    use super::{QueryName, QueryNameParser};
    use crate::parser::{
        test_helpers::{get_buffer_at_question_section, GOOGLE_QUERY},
        wrapped_buffer::WrappedBuffer,
    };
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
    fn writes_domain_name_successfully() -> Result<(), Box<dyn Error>> {
        let expected_domain_name = "google.com";
        let mut actual_domain_name = String::new();

        let mut buffer = WrappedBuffer::new();

        QueryName::write(&mut buffer, "google.com")?;
        buffer.seek(0)?;
        QueryName::read(&mut buffer, &mut actual_domain_name)?;

        assert_eq!(expected_domain_name, actual_domain_name);
        Ok(())
    }

    #[test]
    #[ignore = "Need to create a packet exhibiting this scenario in a hex editor or something."]
    fn parsing_fails_for_packet_with_too_many_jumps() -> Result<(), Box<dyn Error>> {
        todo!();
    }
}
