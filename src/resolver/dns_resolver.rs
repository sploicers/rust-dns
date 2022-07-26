use crate::parser::{DnsPacket, WrappedBuffer};
use std::{
    error::Error,
    net::{Ipv4Addr, UdpSocket},
};

pub struct DnsResolver {
    ip: Ipv4Addr,
    socket: UdpSocket,
    port: u16,
}

impl DnsResolver {
    pub fn new(ip: Ipv4Addr, port: u16) -> Result<DnsResolver, Box<dyn Error>> {
        let socket = UdpSocket::bind((ip, port))?;
        Ok(DnsResolver { ip, socket, port })
    }

    pub fn query(&self, query: &mut DnsPacket) -> Result<DnsPacket, Box<dyn Error>> {
        let mut query_buffer = WrappedBuffer::new();
        let mut response_buffer = WrappedBuffer::new();
        query.write(&mut query_buffer)?;

        self.socket.send_to(
            &query_buffer.raw_buffer[0..query_buffer.pos()],
            (self.ip, self.port),
        )?;
        self.socket.recv_from(&mut response_buffer.raw_buffer)?;
        Ok(DnsPacket::from_buffer(&mut response_buffer)?)
    }
}
