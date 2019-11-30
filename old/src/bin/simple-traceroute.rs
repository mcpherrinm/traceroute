extern crate traceroute;
use std::io::net::ip::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
  let req=  traceroute::TraceRequest{
    tries: 3,
    hop_limit: 30,
    destination: Ipv4Addr(8,8,8,8)
  };
  
  let res = req.run();

  println!("{}", res);


}
