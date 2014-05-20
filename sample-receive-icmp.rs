extern crate libc;
extern crate native;
use libc::{c_int, c_void, socket, AF_INET, sockaddr_storage};
use native::io::net::sockaddr_to_addr;
use std::io::net::ip::SocketAddr;
static SOCK_RAW: c_int = 3;
static IPPROTO_ICMP: c_int = 1;

fn recvfrom<'buf>(sock: c_int, buf: &'buf mut [u8]) -> (&'buf mut [u8], SocketAddr) {
  let mut storage: sockaddr_storage = unsafe { std::mem::init() };
  let storagep = &mut storage as *mut _ as *mut libc::sockaddr;
  let mut addrlen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;

  let bytes = unsafe { libc::recvfrom(sock,
                 buf.as_mut_ptr() as *mut c_void,
                 buf.len() as u64, 
                 0, storagep, &mut addrlen) };

  (buf.mut_slice_to(bytes as uint),
   sockaddr_to_addr(&storage, addrlen as uint).unwrap())
}

fn main() {
  let handle = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_ICMP) };
  println!("{}", handle);
  let mut bufferator = [0, ..2048];
  loop {
    let (buf, from) = recvfrom(handle, bufferator.as_mut_slice());
    println!("from {}, data:\n{}", from, buf);
  }
}
