extern crate libc;
extern crate native;
extern crate packet;
use std::io::net::udp::UdpSocket;
use std::io::net::ip::{IpAddr, Ipv4Addr, SocketAddr};

/*fn print_ip(reader: &mut Reader) {
  let iphdr = packet::parse_ip(reader).unwrap();
  println!("header: src {} dst {} ttl {}", iphdr.src, iphdr.dst, iphdr.ttl);
  match iphdr.protocol {
    1 => print_icmp(reader),
    4 => print_ip(reader),
    //6 => print_tcp(reader),
    17 => print_udp(reader),
    x => println!("unknown protocol {}", x),
  }
}*/

struct Hop {
  ip: IpAddr,
  time: uint,
}

#[allow(experimental)]
// Need experimental to set_tll, which is key for this to work.
fn send_trace(ip: IpAddr, seed: u64) {
  let addr = SocketAddr { ip: Ipv4Addr(0, 0, 0, 0), port: seed as u16};
  let mut dest = SocketAddr { ip: ip, port: 12345 };
  let mut socket = UdpSocket::bind(addr).unwrap();

  let mut buf = [1,2,3,4,5,6,7,8];
  for ttl in range(1, 30) {
    socket.set_ttl(ttl).unwrap();
    dest.port = ttl as u16;
    buf[0] = ttl as u8;
    socket.sendto(buf, dest).unwrap();
    // Probably want to put the time in the buffer here.
  }
}

fn main() {
  let socket = packet::rawsocket::RawSocket::icmp_sock().unwrap();

  spawn( proc() {send_trace(Ipv4Addr(8,8,8,8), 12345)} );

  let mut buffer = [0, ..4096];
  let mut responses: Vec<Option<Hop>> = Vec::new();
  for _ in range(0, 30) {
    responses.push(None);
  }
  loop {
    let buf = socket.recvfrom(buffer.as_mut_slice());
    let ip = packet::Ip::new(buf);
    println!("Snarfed IP:");
    ip.print();
    if (ip.version(), ip.protocol()) != (4, 1) { continue };

    let icmp = packet::Icmp::new(ip.payload());
    icmp.print();

    match icmp.icmp_type() {
      11 => { // TTL exceeded
        let innerip = packet::Ip::new(icmp.payload());
        innerip.print();
        let udp = packet::Udp::new(innerip.payload());
        udp.print();
        let ttl = udp.dstport();
        *responses.get_mut(ttl as uint) = Some( Hop { ip: ip.source(), time: 0 } );
        for i in responses.iter() {
          match *i {
            Some(hop) => println!("{} {} {} {}", hop.ip, hop.time, '*', '*'),
            None => (), //println!("-"),
          }
        }
        println!("-");
      },
      _ => continue,
    }
  }
}
