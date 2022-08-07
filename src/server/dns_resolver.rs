use super::wrapped_socket::WrappedSocket;
use crate::parser::{DnsPacket, DnsQuestion, QueryType, ResultCode};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};

const REMOTE_SERVER_IP: Ipv4Addr = Ipv4Addr::new(8, 8, 8, 8); // Still just forward queries to Google's DNS resolver for now.
const REMOTE_SOCKET_PORT: u16 = 53;
const LOCAL_SOCKET_PORT: u16 = 4000;

pub fn start_listening(port: u16) -> Result<(), Box<dyn Error>> {
    let local_addr: SocketAddr = (Ipv4Addr::UNSPECIFIED, port).into();
    let remote_addr: SocketAddr = (Ipv4Addr::UNSPECIFIED, REMOTE_SOCKET_PORT).into();
    let mut socket = WrappedSocket::new(local_addr, remote_addr);

    log::info!("Server listening on port {}.", port);
    loop {
        answer_query(&mut socket)?;
    }
}

fn answer_query(socket: &mut WrappedSocket) -> Result<(), Box<dyn Error>> {
    log::info!("Waiting to receive a query...");
    let mut query = DnsPacket::read(socket)?;
    log::info!("Received query:\n{}", query);

    let mut response = DnsPacket::new();
    response.header.id = query.header.id;
    response.header.num_questions = 1;
    response.header.recursion_desired = true;
    response.header.recursion_available = true;
    response.header.response = true;

    match query.questions.pop() {
        Some(question) => match resolve_name(question.name.as_str(), question.query_type) {
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
        // Incoming query packet is malformed - contains no question records.
        _ => response.header.rescode = ResultCode::FORMERR,
    };
    response.write(socket)?;
    log::info!("Sent response:\n{}", response);
    Ok(())
}

fn resolve_name(name: &str, query_type: QueryType) -> Result<DnsPacket, Box<dyn Error>> {
    let local_addr = (Ipv4Addr::UNSPECIFIED, LOCAL_SOCKET_PORT);
    let remote_addr = (REMOTE_SERVER_IP, REMOTE_SOCKET_PORT);
    let mut socket = WrappedSocket::new(local_addr.into(), remote_addr.into());
    let mut packet = DnsPacket::new();

    packet.header.id = 451;
    packet.header.num_questions = 1;
    packet.header.recursion_desired = true;

    packet.questions.push(DnsQuestion {
        name: name.to_string(),
        query_type,
    });

    packet.write(&mut socket)?;
    Ok(DnsPacket::read(&mut socket)?)
}

#[cfg(test)]
mod tests {
    use super::resolve_name;
    use crate::parser::{DnsPacket, DnsRecord, QueryType};
    use std::error::Error;

    #[test]
    fn can_answer_dns_query() -> Result<(), Box<dyn Error>> {
        let expected_domain = "google.com";
        let response: DnsPacket = resolve_name(expected_domain, QueryType::A)?;
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
