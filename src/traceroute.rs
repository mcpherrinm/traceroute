extern crate packet;
use std::io::net::ip::{IpAddr, Ipv4Addr, SocketAddr};
use std::io::net::udp::UdpSocket;

pub struct Hop {
  time: uint,
  where: IpAddr,
}

pub struct TraceRequest {
  /// The number of packets we want to send to each host along the way.
  tries: uint,
  /// How far away we're going to look
  hop_limit: uint,
  destination: IpAddr,
  // Todo: progress callback.  For now, hardcode printing?
}

impl Send for TraceRequest {}

impl TraceRequest {
  /// Runs the traceroute, sending in a new task and receiving in
  /// this one.
  pub fn run(&self) -> Vec<Vec<Hop>> {
    // Start listeclone().ning on a socket first:
    let listener = packet::rawsocket::RawSocket::icmp_sock().unwrap();

    let sender = *self.clone();
    std::task::spawn(proc() { sender.send_all_probes() });

    let mut buf = [8, ..9200];
    loop {
      let pkt = listener.recvfrom(buf.as_mut_slice());
    }


   Vec::new()
  }

  // Syncronously send all the probes
  #[allow(experimental)]
  fn send_all_probes(&self) {
    let addr = SocketAddr { ip: Ipv4Addr(0, 0, 0, 0), port: 12345};
    let mut dest = SocketAddr { ip: self.destination, port: 12345};
    let mut socket = UdpSocket::bind(addr).unwrap();

    let mut buf = [1,2,3,4,5,6,7,8];
    for ttl in range(1, self.hop_limit) {
      for _ in range(0, self.tries) {
        socket.set_ttl(ttl as int).unwrap();
        dest.port = ttl as u16;
        buf[0] = ttl as u8;
        socket.sendto(buf, dest).unwrap();
        // Probably want to put the time in the buffer here.
      }
    }
  }
}

pub struct TraceResponsePkt<'a> {
  buf: &'a [u8]
}

impl<'b> TraceResponsePkt<'b> {
  fn new<'a>(buf: &'a [u8]) -> Option<TraceResponsePkt<'a>> {
    Some(TraceResponsePkt{buf: buf})
  }
}
