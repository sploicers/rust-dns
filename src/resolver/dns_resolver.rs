use crate::parser::{DnsPacket, DnsQuestion, QueryType, ResultCode, WrappedBuffer};
use std::{
    error::Error,
    net::{Ipv4Addr, UdpSocket},
};

const SOCKET_PORT: u16 = 53;

pub struct DnsResolver {
    ip: Ipv4Addr,
    socket: UdpSocket,
}

impl DnsResolver {
    pub fn new(ip: Ipv4Addr, port: u16) -> Result<DnsResolver, Box<dyn Error>> {
        match UdpSocket::bind((Ipv4Addr::UNSPECIFIED, port)) {
            Ok(socket) => Ok(DnsResolver { ip, socket }),
            _ => panic!("Failed to bind socket! (ip: {}, port: {})", ip, port),
        }
    }

    pub fn start_listening(&self) -> Result<(), Box<dyn Error>> {
        loop {
            self.answer_query()?;
        }
    }

    fn answer_query(&self) -> Result<(), Box<dyn Error>> {
        let mut query_buffer = WrappedBuffer::new();
        let (_, address) = self.socket.recv_from(&mut query_buffer.as_slice()?)?;

        let mut query = DnsPacket::from_buffer(&mut query_buffer)?;
        let mut response = DnsPacket::new();
        response.header.id = query.header.id;
        response.header.num_questions = 1;
        response.header.recursion_desired = true;
        response.header.recursion_available = true;
        response.header.response = true;

        match query.questions.pop() {
            Some(question) => match self.query(question.name.as_str(), question.query_type) {
                Ok(downstream_result) => {
                    response.questions.push(question);
                    response.header.rescode = downstream_result.header.rescode;

                    for answer in downstream_result.answers {
                        response.answers.push(answer);
                    }
                    for record in downstream_result.authorities {
                        response.authorities.push(record);
                    }
                    for record in downstream_result.additional_records {
                        response.additional_records.push(record);
                    }
                }
                // Got an error response from downstream.
                _ => response.header.rescode = ResultCode::SERVFAIL,
            },
            // Incoming query packet contains no questions - must be malformed.
            _ => response.header.rescode = ResultCode::FORMERR,
        };

        let mut response_buffer = WrappedBuffer::new();
        response.write(&mut response_buffer)?;

        let size = response_buffer.pos();
        self.socket
            .send_to(response_buffer.get_slice(0, size)?, address)?;

        Ok(())
    }

    fn query(&self, name: &str, query_type: QueryType) -> Result<DnsPacket, Box<dyn Error>> {
        let mut query_buffer = WrappedBuffer::new();
        let mut response_buffer = WrappedBuffer::new();
        let mut packet = DnsPacket::new();

        packet.header.id = 0451;
        packet.header.num_questions = 1;
        packet.header.recursion_desired = true;
        packet.questions.push(DnsQuestion {
            name: name.to_string(),
            query_type,
        });
        packet.write(&mut query_buffer)?;

        self.socket.send_to(
            query_buffer.get_slice(0, query_buffer.pos())?,
            (self.ip, SOCKET_PORT),
        )?;
        self.socket.recv_from(&mut response_buffer.as_slice()?)?;
        Ok(DnsPacket::from_buffer(&mut response_buffer)?)
    }
}

#[cfg(test)]
mod tests {
    use super::DnsResolver;
    use crate::parser::{DnsPacket, DnsRecord, QueryType};
    use std::{error::Error, net::Ipv4Addr};

    #[test]
    fn can_answer_dns_query() -> Result<(), Box<dyn Error>> {
        let resolver = DnsResolver::new(Ipv4Addr::new(8, 8, 8, 8), 8000)?;
        let expected_domain = "google.com";
        let response: DnsPacket = resolver.query(expected_domain, QueryType::A)?;
        let answers = response.answers;

        match answers.first() {
            Some(answer) => match answer {
                DnsRecord::A { domain, .. } => {
                    assert_eq!(domain, expected_domain);
                }
                _ => panic!("Expected DNS query answer to be an A record, got unknown type."),
            },
            _ => panic!("Expected to receive an answer to DNS query."),
        };

        Ok(())
    }
}
