use std::{
    io::{Read, Result, Write},
    net::{SocketAddr, UdpSocket},
};

pub struct RemoteSocket {
    raw_socket: UdpSocket,
    remote_addr: SocketAddr,
}

impl RemoteSocket {
    pub fn bind(local_addr: SocketAddr, remote_addr: SocketAddr) -> RemoteSocket {
        RemoteSocket {
            raw_socket: UdpSocket::bind(local_addr)
                .expect(format!("Failed to bind socket (port: {})", local_addr.port()).as_str()),
            remote_addr,
        }
    }
}

impl Read for RemoteSocket {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let (size, _) = self.raw_socket.recv_from(buf)?;
        Ok(size)
    }
}

impl Write for RemoteSocket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(self.raw_socket.send_to(buf, self.remote_addr)?)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}
