use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    net::{SocketAddr, UdpSocket},
};

// A socket which cannot be written to until after it has been read from.
pub struct LocalSocket {
    raw_socket: UdpSocket,
    last_received_addr: Option<SocketAddr>,
}

impl LocalSocket {
    pub fn bind(addr: SocketAddr) -> LocalSocket {
        LocalSocket {
            raw_socket: UdpSocket::bind(addr)
                .expect(format!("Failed to bind socket (port: {})", addr.port()).as_str()),
            last_received_addr: None,
        }
    }
}

impl Read for LocalSocket {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let (size, origin) = self.raw_socket.recv_from(buf)?;
        self.last_received_addr = Some(origin);
        Ok(size)
    }
}

impl Write for LocalSocket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if let Some(respond_addr) = self.last_received_addr {
            let size = self.raw_socket.send_to(buf, respond_addr)?;
            self.last_received_addr = None;
            Ok(size)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Fatal - attempted to write response before receiving a request!",
            ))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}
