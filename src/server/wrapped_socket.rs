use std::{
    io::{Read, Result, Write},
    net::{SocketAddr, UdpSocket},
};

pub struct WrappedSocket {
    raw_socket: UdpSocket,
    remote_addr: SocketAddr,
    last_received_addr: Option<SocketAddr>,
}

impl WrappedSocket {
    pub fn new(local_addr: SocketAddr, remote_addr: SocketAddr) -> WrappedSocket {
        WrappedSocket {
            raw_socket: UdpSocket::bind(local_addr)
                .expect(format!("Failed to bind socket (port: {})", local_addr.port()).as_str()),

            remote_addr,
            last_received_addr: None,
        }
    }
}

impl Read for WrappedSocket {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let (size, origin) = self.raw_socket.recv_from(buf)?;
        self.last_received_addr = Some(origin);
        Ok(size)
    }
}

impl Write for WrappedSocket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let send_addr = if let Some(addr) = self.last_received_addr {
            addr
        } else {
            self.remote_addr
        };

        let size = self.raw_socket.send_to(buf, send_addr)?;
        self.last_received_addr = None;
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}
