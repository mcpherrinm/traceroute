use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};

fn main(){
  let addr = SocketAddr { ip: Ipv4Addr(0, 0,  0, 0), port: 12345 };
  let dest = SocketAddr { ip: Ipv4Addr(129,97,134,34), port: 12345 };
  let mut socket = UdpSocket::bind(addr).unwrap();

  let buf = [1,2,3,4,5,6,7,8];
  for ttl in range(1, 30) {
    socket.set_ttl(ttl).unwrap();
    socket.sendto(buf, dest).unwrap();
  }
}
