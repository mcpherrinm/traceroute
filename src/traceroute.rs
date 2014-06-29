extern crate packet;
use std::io::net::ip::{IpAddr, Ipv4Addr, SocketAddr};
use std::io::net::udp::UdpSocket;

#[deriving(Show)]
pub struct Hop {
  pub where: IpAddr,
  pub time: uint,
}

pub struct TraceRequest {
  /// The number of packets we want to send to each host along the way.
  pub tries: uint,
  /// How far away we're going to look
  pub hop_limit: uint,
  pub destination: IpAddr,
  // Todo: progress callback.  For now, hardcode printing?
}

impl Send for TraceRequest {}

impl TraceRequest {
  /// Runs the traceroute, sending in a new task and receiving in
  /// this one.
  pub fn run(&self) -> Vec<Vec<Hop>> {
    let mut results = Vec::new();
    for _ in range(0, self.hop_limit) {
      results.push(Vec::new());
    }
    // Start listening on a socket first:
    let listener = packet::rawsocket::RawSocket::icmp_sock().unwrap();
    listener.timeout(2);

    let sender = *self.clone();
    std::task::spawn(proc() { sender.send_all_probes() });

    let mut buf = [8, ..9200];
    let mut recv_count = 0;
    let mut failcount = 0;
    loop {
      let pkt = listener.recvfrom(buf.as_mut_slice());
      if pkt.is_none() {
        // assuming failures are timeouts...
        failcount += 1;
        if failcount > 5 { break }
        continue
      }
      match TraceResponsePkt::new(pkt.unwrap()) {
        None => continue, // Not a packet we are about
        Some(resp) => {
          recv_count += 1;
          println!("{}: {} @ {}", resp.from(), resp.dist(), resp.time());
          results.get_mut(resp.dist()).push(Hop{time: resp.time(), where: resp.from()});
          if recv_count >= self.hop_limit * self.tries {
            break;
          }
        }
      }
    }

    results
  }

  // Syncronously send all the probes
  #[allow(experimental)]
  fn send_all_probes(&self) {
    let addr = SocketAddr { ip: Ipv4Addr(0, 0, 0, 0), port: 12345};
    let mut dest = SocketAddr { ip: self.destination, port: 0};
    let mut socket = UdpSocket::bind(addr).unwrap();

    let mut buf = [1,2,3,4,5,6,7,8];
    for _ in range(0, self.tries) {
      for ttl in range(1, self.hop_limit) {
        socket.set_ttl(ttl as int).unwrap();
        dest.port = (ttl as u16) + 50000;
        buf[0] = ttl as u8;
        socket.sendto(buf, dest).unwrap();
        // Probably want to record the time somehow.
      }
    }
  }
}

pub struct TraceResponsePkt<'a> {
  ip: packet::parser::Ip<'a>,
  icmp: packet::parser::Icmp<'a>,
  inner_ip: packet::parser::Ip<'a>,
  udp: packet::parser::Udp<'a>,
}

impl<'b> TraceResponsePkt<'b> {
  fn new<'a>(buf: &'a [u8]) -> Option<TraceResponsePkt<'a>> {
    let ip = packet::parser::Ip::new(buf);
    if (ip.version(), ip.protocol()) != (4, 1) { return None; }

    let icmp = packet::parser::Icmp::new(ip.payload());
    if icmp.icmp_type() != 11 || icmp.icmp_type() != 3 { return None; }

    let inner_ip = packet::parser::Ip::new(icmp.payload());
    if (ip.version(), ip.protocol()) != (4, 17) { return None; }

    let udp = packet::parser::Udp::new(inner_ip.payload());
    // if ... this UDP packet is one we sent
    // surely involving whatever scheme for recording time we have.

    Some(TraceResponsePkt{ip: ip, icmp: icmp, inner_ip: inner_ip, udp: udp})
  }

  pub fn from(&self) -> IpAddr {
    self.ip.source()
  }

  pub fn dist(&self) -> uint {
    self.inner_ip.ttl() as uint
  }

  pub fn time(&self) -> uint {
    0
  }
}
