use std::{error::Error, fmt::Display, io::Read};

use super::{
    dns_header::DnsHeader, dns_question::DnsQuestion, dns_record::DnsRecord, query_type::QueryType,
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
    fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additional_records: Vec::new(),
        }
    }

    pub fn from_reader(reader: &mut dyn Read) -> Result<DnsPacket, Box<dyn Error>> {
        let mut buffer = WrappedBuffer::new();
        reader.read(&mut buffer.raw_buffer)?;
        Ok(DnsPacket::from_buffer(&mut buffer)?)
    }

    fn from_buffer(buffer: &mut WrappedBuffer) -> Result<DnsPacket, String> {
        let mut packet = DnsPacket::new();
        packet.header.read(buffer)?;

        for _ in 0..packet.header.num_questions {
            let mut question = DnsQuestion::new("".into(), QueryType::UNKNOWN(0));
            question.read(buffer)?;
            packet.questions.push(question);
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
