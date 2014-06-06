extern crate libc;
extern crate native;
use std::io::BufReader;
use std::io::net::udp::UdpSocket;
use std::io::net::ip::{IpAddr, Ipv4Addr, SocketAddr};
use libc::{c_int, c_void, socket, AF_INET, sockaddr_storage};
use native::io::net::sockaddr_to_addr;

mod packet;

static SOCK_RAW: c_int = 3;
static IPPROTO_ICMP: c_int = 1;

fn recvfrom<'buf>(sock: c_int, buf: &'buf mut [u8]) -> (&'buf mut [u8], SocketAddr) {
  let mut storage: sockaddr_storage = unsafe { std::mem::zeroed() };
  let storagep = &mut storage as *mut _ as *mut libc::sockaddr;
  let mut addrlen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;

  let bytes = unsafe { libc::recvfrom(sock,
                 buf.as_mut_ptr() as *mut c_void,
                 buf.len() as u64, 
                 0, storagep, &mut addrlen) };

  (buf.mut_slice_to(bytes as uint),
   sockaddr_to_addr(&storage, addrlen as uint).unwrap())
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
fn main() {
  let handle = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_ICMP) };
  if handle < 0 {
    println!("{}, could't open handle. root?", handle);
    return;
  }

  spawn(proc() {
    let addr = SocketAddr { ip: Ipv4Addr(0, 0,  0, 0), port: 12345 };
    let mut dest = SocketAddr { ip: Ipv4Addr(8,8,8,8), port: 12345 };
    let mut socket = UdpSocket::bind(addr).unwrap();

    let mut buf = [1,2,3,4,5,6,7,8];
    for ttl in range(1, 30) {
      socket.set_ttl(ttl).unwrap();
      dest.port = ttl as u16;
      buf[0] = ttl as u8;
      socket.sendto(buf, dest).unwrap();
      // Probably want to put the time in the buffer here.
    }
  });

  let mut bufferator = [0, ..4096];
  let mut responses: Vec<Option<Hop>> = Vec::new();
  for _ in range(0, 30) {
    responses.push(None);
  }
  loop {
    let (buf, _) = recvfrom(handle, bufferator.as_mut_slice());
    let reader = &mut BufReader::new(buf) as &mut Reader;
    let ip = packet::parse_ip(reader);
    match ip { Err(_) => continue, _ => () };
    let ip = ip.ok().unwrap();
    if ip.protocol != 1 { continue; }; // icmp
    let icmp = packet::parse_icmp(reader);
    match icmp { Err(_) => continue, _ => () };
    let icmp = icmp.ok().unwrap();
    match icmp.icmp_type {
      11 => { // TTL exceeded
        let innerip = packet::parse_ip(reader);
        match innerip { Err(_) => continue, _ => () };
        innerip.ok().unwrap(); // verify this is actually from me
        let udp = packet::parse_udp(reader);
        match udp { Err(_) => continue, _ => () };
        let udp = udp.ok().unwrap();
        let ttl = udp.dstport;
        {
        let entry = responses.get_mut(ttl as uint);
        *entry = Some( Hop { ip: ip.src, time: 0 } );
        }
        for i in responses.iter() {
          match *i {
            Some(hop) => println!("{} {} {} {}", hop.ip, hop.time, '*', '*'),
            None => println!("-"),
          }
        }
      },
      _ => continue,
    }
  }
}
