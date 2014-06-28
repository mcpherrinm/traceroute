extern crate std;
use std::io::net::ip::{IpAddr, Ipv4Addr};

pub struct Ip<'a> {buf: &'a [u8] }

impl<'a> Ip<'a> {
  pub fn new<'a>(buf: &'a [u8]) -> Ip<'a> {
    // We could do any safety-required checks here,
    // and use unsafe buffer accesses everywhere else.
    // and return Option
    Ip{buf: buf}
  }

  pub fn version(&self) -> u8 { self.buf[0] >> 4 }

  pub fn hdr_len(&self) -> u8 { self.buf[0] & 0x0F }

  pub fn hdr_bytes(&self) -> u8 { self.hdr_len() * 4 }

  pub fn tos(&self) -> u8 { self.buf[0] }

  pub fn total_len(&self) -> u16 { (self.buf[2] as u16) << 8 | self.buf[3] as u16 }

  pub fn frag_id(&self) -> u16 { (self.buf[4] << 8) as u16 | (self.buf[5] as u16) }

  pub fn offset(&self) -> u16 { ((self.buf[6] & 0b001_1111) << 8) as u16 | self.buf[7] as u16 }

  pub fn ttl(&self) -> u8 { self.buf[8] }

  pub fn protocol(&self) -> u8 { self.buf[9] }

  pub fn checksum(&self) -> u16 { (self.buf[10] << 8) as u16 | self.buf[11] as u16 }

  pub fn source(&self) -> IpAddr {
    Ipv4Addr(self.buf[12], self.buf[13], self.buf[14], self.buf[15])
  }

  pub fn dest(&self) -> IpAddr {
    Ipv4Addr(self.buf[16], self.buf[17], self.buf[18], self.buf[19])
  }

  // Eh, todo. Iterator over IpOptions?
  //pub fn options(&self) -> ... {  }

  pub fn payload(&self) -> &'a [u8] {
    if self.total_len() as uint > self.buf.len() {
      self.buf.slice_from(self.hdr_bytes() as uint)
    } else {
      self.buf.slice(self.hdr_bytes() as uint, self.total_len() as uint)
    }
  }
  pub fn print(&self) {
    println!("Ip  | ver {} | {} | Tos {} | Len {}  |", self.version(), self.hdr_len(), self.tos(), self.total_len());
    println!("    | FId {}    |   off {} |", self.frag_id(), self.offset());
    println!("    | ttl {} | proto {} | sum {} |", self.ttl(), self.protocol(), self.checksum());
    println!("    | Src {}   | Dst {} |", self.source(), self.dest());
  }
}

pub struct Icmp<'a> {buf: &'a [u8] }

impl<'a> Icmp<'a> {
  pub fn new<'a>(buf: &'a [u8]) -> Icmp<'a> {
    Icmp{buf: buf}
  }

  pub fn icmp_type(&self) -> u8 {
    self.buf[0]
  }

  pub fn code(&self) -> u8 {
    self.buf[1]
  }

  pub fn checksum(&self) -> u16 {
    ((self.buf[2] as u16) << 8) | (self.buf[3] as u16)
  }

  pub fn data(&self) -> &'a [u8] {
    self.buf.slice(4, 8)
  }

  pub fn payload(&self) -> &'a [u8] {
    self.buf.slice_from(8)
  }
  pub fn print(&self) {
    println!("Icmp| type : {}  | code : {}  |", self.icmp_type(), self.code() );
    println!("    | length: {}  | Chksm: {}  |", self.payload().len(), self.checksum() );
  }
}

#[deriving(Show)]
pub struct Udp<'a> {buf: &'a [u8] }
/*
  pub srcport: u16, pub dstport: u16, pub length: u16, pub checksum: u16
*/

impl<'a> Udp<'a> {
  pub fn new<'a>(buf: &'a [u8]) -> Udp<'a> {
    Udp{buf: buf}
  }

  pub fn srcport(&self) -> u16 {
    ((self.buf[0] as u16) << 8) | (self.buf[1] as u16)
  }

  pub fn dstport(&self) -> u16 {
    ((self.buf[2] as u16) << 8) | (self.buf[3] as u16)
  }

  /// The length of the packet, including header, in bytes
  pub fn length(&self) -> u16 {
    ((self.buf[4] as u16) << 8) | (self.buf[5] as u16)
  }

  pub fn checksum(&self) -> u16 {
    ((self.buf[6] as u16) << 8) | (self.buf[7] as u16)
  }

  pub fn payload(&self) -> &'a [u8] {
    if self.buf.len() > self.length() as uint {
      self.buf.slice(8, self.length() as uint)
    } else {
      self.buf.slice_from(8)
    }
  }

  pub fn print(&self) {
    println!("Udp | Srcp: {}  | Dstp: {}  |", self.srcport(), self.dstport() );
    println!("    | length: {}  | Chksm: {}  |", self.length(), self.checksum() );
  }
}
