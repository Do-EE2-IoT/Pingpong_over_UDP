pub mod UDP {
    use std::{
        io::{self, Error},
        net::SocketAddr,
    };

    use bincode::ErrorKind;
    use std::net::{Ipv4Addr, SocketAddrV4};
    use tokio::net::UdpSocket;

    pub struct UdpStream {
        socket: UdpSocket,
    }

    pub fn make_ipv4_addr(ipv4: (u8, u8, u8, u8), port: u16) -> SocketAddrV4 {
        SocketAddrV4::new(Ipv4Addr::new(ipv4.0, ipv4.1, ipv4.2, ipv4.3), port)
    }

    impl UdpStream {
        pub async fn listen(addr: &str) -> Result<UdpStream, Error> {
            let socket = UdpSocket::bind(addr).await.unwrap();
            Ok(UdpStream { socket })
        }

        pub async fn send(&mut self, address: &SocketAddrV4, data: Vec<u8>) -> io::Result<()> {
            self.socket.send_to(&data, address).await?;
            Ok(())
        }

        pub async fn read(&mut self) -> io::Result<Vec<u8>> {
            let mut buf = vec![0;2048];
            let len = self.socket.recv(&mut buf).await?;
            buf.truncate(len);
            Ok(buf.to_vec())
        }

        pub async fn broadcast_enable(&self, enable: bool) {
            self.socket.set_broadcast(enable).unwrap();
        }
        pub async fn broadcast_to_port(
            &mut self,
            port: u16,
            data: Vec<u8>,
        ) -> Result<(), io::ErrorKind> {
            if self.socket.broadcast().unwrap() {
                let broadcast_address = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), port);
                self.socket.send_to(&data, broadcast_address).await.unwrap();
                Ok(())
            } else {
                Err(io::ErrorKind::PermissionDenied)
            }
        }
    }
}
