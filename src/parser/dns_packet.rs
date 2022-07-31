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
        let mut packet = DnsPacket::new();

        reader.read(&mut buffer.as_slice()?)?;
        packet.header.read(&mut buffer)?;

        for _ in 0..packet.header.num_questions {
            packet.questions.push(DnsQuestion::read(&mut buffer)?);
        }
        for _ in 0..packet.header.num_answers {
            packet.answers.push(DnsRecord::read(&mut buffer)?);
        }
        for _ in 0..packet.header.num_authorities {
            packet.authorities.push(DnsRecord::read(&mut buffer)?);
        }
        for _ in 0..packet.header.num_additional {
            packet
                .additional_records
                .push(DnsRecord::read(&mut buffer)?);
        }
        Ok(packet)
    }

    pub fn write<T: Write>(&mut self, writer: &mut T) -> Result<usize, Box<dyn Error>> {
        let mut buffer = WrappedBuffer::new();
        self.write_header(&mut buffer)?;
        self.write_records(&mut buffer)?;
        Ok(writer.write(buffer.get_slice(0, buffer.pos())?)?)
    }

    fn write_header(&mut self, buffer: &mut WrappedBuffer) -> Result<(), Box<dyn Error>> {
        self.header.num_questions = self.questions.len() as u16;
        self.header.num_answers = self.answers.len() as u16;
        self.header.num_authorities = self.authorities.len() as u16;
        self.header.num_additional = self.additional_records.len() as u16;
        self.header.write(buffer)?;
        Ok(())
    }

    fn write_records(&mut self, buffer: &mut WrappedBuffer) -> Result<(), Box<dyn Error>> {
        for question in &self.questions {
            question.write(buffer)?;
        }
        for record in &self.answers {
            record.write(buffer)?;
        }
        for record in &self.authorities {
            record.write(buffer)?;
        }
        for record in &self.additional_records {
            record.write(buffer)?;
        }
        Ok(())
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
    use crate::parser::test_helpers::{open_test_file, GOOGLE_QUERY};
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
        let mut file = open_test_file(String::from(GOOGLE_QUERY))?;
        Ok(DnsPacket::read(&mut file)?)
    }
}
