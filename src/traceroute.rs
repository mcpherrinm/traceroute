extern crate libc;
extern crate native;
extern crate packet;
use std::io::net::udp::UdpSocket;
use std::io::net::ip::{IpAddr, Ipv4Addr, SocketAddr};
use libc::{c_int, c_void, socket, AF_INET, sockaddr_storage};

static SOCK_RAW: c_int = 3;
static IPPROTO_ICMP: c_int = 1;

fn recvfrom<'buf>(sock: c_int, buf: &'buf mut [u8]) -> &'buf mut [u8] {
  let mut storage: sockaddr_storage = unsafe { std::mem::zeroed() };
  let storagep = &mut storage as *mut _ as *mut libc::sockaddr;
  let mut addrlen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;

  let bytes = unsafe { libc::recvfrom(sock,
                 buf.as_mut_ptr() as *mut c_void,
                 buf.len() as u64, 
                 0, storagep, &mut addrlen) };

  buf.mut_slice_to(bytes as uint)
}

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
  let handle = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_ICMP) };
  if handle < 0 {
    println!("{}, could't open handle. root?", handle);
    return;
  }

  spawn( proc() {send_trace(Ipv4Addr(8,8,8,8), 12345)} );

  let mut buffer = [0, ..4096];
  let mut responses: Vec<Option<Hop>> = Vec::new();
  for _ in range(0, 30) {
    responses.push(None);
  }
  loop {
    let buf = recvfrom(handle, buffer.as_mut_slice());
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
