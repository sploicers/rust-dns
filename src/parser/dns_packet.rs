use std::{
    error::Error,
    fmt::Display,
    io::{Read, Write},
};

use super::{
    dns_header::DnsHeader, dns_question::DnsQuestion, dns_record::DnsRecord,
    wrapped_buffer::WrappedBuffer,
};

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub additional_records: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additional_records: Vec::new(),
        }
    }

    pub fn read<T: Read>(reader: &mut T) -> Result<DnsPacket, Box<dyn Error>> {
        let mut buffer = WrappedBuffer::new();
        reader.read(&mut buffer.as_slice()?)?;
        Ok(DnsPacket::from_buffer(&mut buffer)?)
    }

    pub fn write<T: Write>(&mut self, writer: &mut T) -> Result<usize, Box<dyn Error>> {
        let mut buffer = WrappedBuffer::new();
        self.header.num_questions = self.questions.len() as u16;
        self.header.num_answers = self.answers.len() as u16;
        self.header.num_authorities = self.authorities.len() as u16;
        self.header.num_additional = self.additional_records.len() as u16;
        self.header.write(&mut buffer)?;

        for question in &self.questions {
            question.write(&mut buffer)?;
        }
        for record in &self.answers {
            record.write(&mut buffer)?;
        }
        for record in &self.authorities {
            record.write(&mut buffer)?;
        }
        for record in &self.additional_records {
            record.write(&mut buffer)?;
        }
        Ok(writer.write(buffer.as_slice()?)?)
    }

    pub fn from_buffer(buffer: &mut WrappedBuffer) -> Result<DnsPacket, String> {
        let mut packet = DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additional_records: Vec::new(),
        };
        packet.header.read(buffer)?;

        for _ in 0..packet.header.num_questions {
            packet.questions.push(DnsQuestion::read(buffer)?);
        }
        for _ in 0..packet.header.num_answers {
            packet.answers.push(DnsRecord::read(buffer)?);
        }
        for _ in 0..packet.header.num_authorities {
            packet.authorities.push(DnsRecord::read(buffer)?);
        }
        for _ in 0..packet.header.num_additional {
            packet.additional_records.push(DnsRecord::read(buffer)?);
        }
        Ok(packet)
    }
}

impl Display for DnsPacket {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "{:#?}", self.header)?;

        for question in &self.questions {
            writeln!(formatter, "{:#?}", question)?;
        }
        for answer in &self.answers {
            writeln!(formatter, "{:#?}", answer)?;
        }
        for authority in &self.authorities {
            writeln!(formatter, "{:#?}", authority)?;
        }
        for record in &self.additional_records {
            writeln!(formatter, "{:#?}", record)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DnsPacket;
    use crate::parser::test_helpers::{get_buffer_at_beginning, GOOGLE_QUERY};
    use std::error::Error;

    #[test]
    fn actual_question_count_matches_header() -> Result<(), Box<dyn Error>> {
        let packet = read_packet()?;
        assert_eq!(packet.header.num_questions as usize, packet.questions.len());
        Ok(())
    }

    #[test]
    fn actual_answer_count_matches_header() -> Result<(), Box<dyn Error>> {
        let packet = read_packet()?;
        assert_eq!(packet.header.num_answers as usize, packet.answers.len());
        Ok(())
    }

    #[test]
    fn actual_authority_count_matches_header() -> Result<(), Box<dyn Error>> {
        let packet = read_packet()?;
        assert_eq!(
            packet.header.num_authorities as usize,
            packet.authorities.len()
        );
        Ok(())
    }

    #[test]
    fn actual_additional_record_count_matches_header() -> Result<(), Box<dyn Error>> {
        let packet = read_packet()?;
        assert_eq!(
            packet.header.num_additional as usize,
            packet.additional_records.len()
        );
        Ok(())
    }

    fn read_packet() -> Result<DnsPacket, Box<dyn Error>> {
        let mut buffer = get_buffer_at_beginning(String::from(GOOGLE_QUERY))?;
        Ok(DnsPacket::from_buffer(&mut buffer)?)
    }
}
