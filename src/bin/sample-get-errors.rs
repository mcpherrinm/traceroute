#[macro_use] extern crate nix;

use std::net::{SocketAddr, ToSocketAddrs};
use std::net::UdpSocket;
use std::os::unix::io::AsRawFd;

use nix::sys::socket::{ControlMessageOwned, recvmsg, sockopt, setsockopt, MsgFlags};
use nix::sys::uio::IoVec;

fn main() {
    // Assume shh.sh resolves and returns at least 1 address, and only ipv4
    // A real tool would want more error checking here
    let dest: SocketAddr = "ssh.sh:12345".to_socket_addrs().unwrap().next().unwrap();
    println!("Sending packets to {:?}", dest);

    let source: SocketAddr = "0.0.0.0:12345".parse().unwrap();
    let socket = UdpSocket::bind(source).unwrap();

    let rawfd = socket.as_raw_fd();
    setsockopt(rawfd, sockopt::Ipv4RecvErr, &true).unwrap();

    println!("Sending packet with TTL:");
    let buf = [1,2,3,4,5,6,7,8];
    for ttl in 1..30 {
        print!("{}, ", ttl);
        socket.set_ttl(ttl).unwrap();
        socket.send_to(&buf, dest); //.unwrap();
    }
    println!("done");

    // TODO: Properly size buf to fit the packets we send
    let mut buf = vec![0u8; 1500];
    let iov = [IoVec::from_mut_slice(&mut buf)];
    let mut cmsg_space = cmsg_space!(nix::sys::socket::SockExtendedErr, libc::sockaddr_in);

    loop {
        match recvmsg(rawfd, &iov, Some(&mut cmsg_space), MsgFlags::MSG_ERRQUEUE) {
            Ok(recvmsg) => {
                println!("recvmsg.address: {:?}", recvmsg.address);
                println!("recvmsg.flags: {:?}", recvmsg.flags);
                for msg in recvmsg.cmsgs() {
                    match msg {
                        ControlMessageOwned::Ipv4RecvErr(e, addr) => println!("ip err {:?} from {:?}", e, addr),
                        m => println!("unknown message {:?}", m),
                    }
                }
            },
            Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => {
                // TODO: Some logic to decide when we're finished (timeout?)
            },
            Err(e) => println!("error {:?}", e)
        }
    }
}
