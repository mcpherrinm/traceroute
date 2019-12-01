use std::net::{SocketAddr, ToSocketAddrs};
use std::net::UdpSocket;

fn main() {
  // Assume mcpherrin.ca resolves and returns at least 1 address
  // A real tool would want more error checking here
  let dest: SocketAddr = "mcpherrin.ca:12345".to_socket_addrs().unwrap().next().unwrap();

  let source: SocketAddr = "0.0.0.0:12345".parse().unwrap();
  let socket = UdpSocket::bind(source).unwrap();

  let buf = [1,2,3,4,5,6,7,8];
  for ttl in 1..30 {
    socket.set_ttl(ttl).unwrap();
    socket.send_to(&buf, dest).unwrap();
  }
}
