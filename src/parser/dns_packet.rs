use super::{
    dns_header::DnsHeader, dns_question::DnsQuestion, dns_record::DnsRecord, query_type::QueryType,
    wrapped_buffer::WrappedBuffer,
};
use std::io::Result;

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub additional_records: Vec<DnsRecord>,
}

impl DnsPacket {
    pub(crate) fn new() -> DnsPacket {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additional_records: Vec::new(),
        }
    }

    pub fn from_buffer(buffer: &mut WrappedBuffer) -> Result<DnsPacket> {
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
