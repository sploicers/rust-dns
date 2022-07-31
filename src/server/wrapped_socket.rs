use std::{
    io::{Read, Result, Write},
    net::{Ipv4Addr, SocketAddr, UdpSocket},
};

pub struct WrappedSocket {
    raw_socket: UdpSocket,
    remote_addr: SocketAddr,
    last_received_addr: Option<SocketAddr>,
}

impl WrappedSocket {
    pub fn new(port: u16, remote_addr: SocketAddr) -> WrappedSocket {
        WrappedSocket {
            raw_socket: UdpSocket::bind((Ipv4Addr::UNSPECIFIED, port))
                .expect(format!("Failed to bind socket (port: {})", port).as_str()),

            remote_addr,
            last_received_addr: None,
        }
    }
}

impl Read for WrappedSocket {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let (size, addr) = self.raw_socket.recv_from(buf)?;
        self.last_received_addr = Some(addr);
        Ok(size)
    }
}

impl Write for WrappedSocket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let size = self.raw_socket.send_to(
            buf,
            if let Some(addr) = self.last_received_addr {
                addr
            } else {
                self.remote_addr
            },
        )?;
        self.last_received_addr = None;
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}
