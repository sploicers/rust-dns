use crate::parser::{DnsPacket, WrappedBuffer};
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

    pub fn query(&self, query: &mut DnsPacket) -> Result<DnsPacket, Box<dyn Error>> {
        let mut query_buffer = WrappedBuffer::new();
        let mut response_buffer = WrappedBuffer::new();
        query.write(&mut query_buffer)?;

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
    use crate::parser::{DnsPacket, DnsQuestion, DnsRecord, QueryType};
    use std::{error::Error, net::Ipv4Addr};

    #[test]
    fn can_answer_dns_query() -> Result<(), Box<dyn Error>> {
        let resolver = DnsResolver::new(Ipv4Addr::new(8, 8, 8, 8), 8000)?;

        let expected_domain = "google.com";
        let mut query: DnsPacket = build_query_packet(expected_domain.to_string(), QueryType::A);
        let response: DnsPacket = resolver.query(&mut query)?;
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

    fn build_query_packet(name: String, query_type: QueryType) -> DnsPacket {
        let mut query = DnsPacket::new();
        query.header.id = 8541;
        query.header.num_questions = 1;
        query.header.recursion_desired = true;
        query.questions.push(DnsQuestion { name, query_type });

        query
    }
}
