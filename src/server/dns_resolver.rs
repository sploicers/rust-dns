use super::socket::{LocalSocket, RemoteSocket};
use crate::parser::{DnsPacket, DnsQuestion, QueryType, ResultCode};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};

pub fn start_listening(port: u16) -> Result<(), Box<dyn Error>> {
    let mut socket = LocalSocket::bind((Ipv4Addr::UNSPECIFIED, port).into());

    log::info!("Server listening on port {}.", port);
    loop {
        answer_query(&mut socket)?;
    }
}

fn answer_query(socket: &mut LocalSocket) -> Result<(), Box<dyn Error>> {
    log::info!("Waiting to receive a query...");
    let query = DnsPacket::read(socket)?; // This blocks until some bytes can be read from the socket.
    log::info!("Received query:\n{}", query);

    let mut response = DnsPacket::new();
    response.header.id = query.header.id;
    response.header.num_questions = 1;
    response.header.recursion_desired = true;
    response.header.recursion_available = true;
    response.header.response = true;

    match query.questions.first() {
        Some(question) => match resolve_name(question.name.as_str(), question.query_type) {
            Ok(downstream_result) => {
                response.questions.push(question.to_owned());
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
    let local_socket_port = 4000;
    let local_addr: SocketAddr = (Ipv4Addr::UNSPECIFIED, local_socket_port).into();
    // Still just forward requests to Google's DNS for now:
    let remote_addr: SocketAddr = (Ipv4Addr::new(8, 8, 8, 8), 53).into();

    let mut socket = RemoteSocket::bind(local_addr, remote_addr);
    let mut packet = DnsPacket::new();

    packet.header.id = 451;
    packet.header.num_questions = 1;
    packet.header.recursion_desired = true;

    packet.questions.push(DnsQuestion {
        name: name.to_string(),
        query_type,
    });

    packet.write(&mut socket)?; // Send query downstream.
    Ok(DnsPacket::read(&mut socket)?) // Read response from downstream.
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
