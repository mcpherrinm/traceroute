extern crate std;
use std::io::net::ip::{IpAddr, Ipv4Addr};
use std::io::IoError;

pub struct Ip {
  pub tos: u8,
  pub data_length: u16,
  pub fragment_id: u16,
  pub fragment_offset: u16,
  pub flags: u8,
  pub ttl: u8,
  pub protocol: u8,
  pub checksum: u16,
  pub src: IpAddr,
  pub dst: IpAddr,
}

pub fn parse_ip(reader: &mut Reader) -> Result<Ip, IoError> {
  let ver_len = try!(reader.read_byte());
  let ver = ver_len >> 4;
  let hdrlen = ver_len & 0x0F;
  if ver != 4 {
    return Err(IoError{kind: ::std::io::OtherIoError,
                       desc: "Didn't read Ipv4 packet", detail: None } );
  }
  let tos = try!(reader.read_byte());
  let total_len = try!(reader.read_be_u16());
  let frag_id = try!(reader.read_be_u16());
  let offset_and_flags = try!(reader.read_be_u16());
  let flags = (offset_and_flags >> 13) as u8;
  let offset = offset_and_flags & 0b0001_1111_1111_1111;
  let ttl = try!(reader.read_byte());
  let protocol = try!(reader.read_byte());
  let checksum = try!(reader.read_be_u16());
  let src = try!(reader.read_be_u32());
  let dst = try!(reader.read_be_u32());
  let opts = hdrlen - 5;
  for _ in range(0, opts) {
    let _ = reader.read_be_u32();
  }

  Ok(Ip {tos: tos,
         data_length: total_len - (hdrlen*4) as u16, 
         fragment_id: frag_id,
         fragment_offset: offset,
         flags: flags,
         ttl: ttl,
         protocol: protocol,
         checksum: checksum,
         src: Ipv4Addr((src >> 24) as u8,
                       (src >> 16) as u8,
                       (src >> 8) as u8,
                       src as u8),
         dst: Ipv4Addr((dst >> 24) as u8,
                       (dst >> 16) as u8,
                       (dst >> 8) as u8,
                       dst as u8)
  })
}

#[deriving(Show)]
pub struct Icmp {
  pub icmp_type: u8,
  pub code: u8,
  pub checksum: u16,
  pub data: u32,
}

pub fn parse_icmp(reader: &mut Reader) -> Result<Icmp, IoError> {
  Ok( Icmp{ icmp_type: try!(reader.read_byte()),
            code: try!(reader.read_byte()),
            checksum: try!(reader.read_be_u16()),
            data: try!(reader.read_be_u32()) })
}

#[deriving(Show)]
pub struct Udp {
  pub srcport: u16,
  pub dstport: u16,
  pub length: u16,
  pub checksum: u16
}

pub fn parse_udp(reader: &mut Reader) -> Result<Udp, IoError> {
  Ok( Udp{ srcport: try!(reader.read_be_u16()),
           dstport: try!(reader.read_be_u16()),
           length: try!(reader.read_be_u16()),
           checksum: try!(reader.read_be_u16()) } )
}
