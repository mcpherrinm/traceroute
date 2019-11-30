use std::io::BufReader;

static sample_ttl: &'static [u8] = &[ 69, 0, 0, 96, 166, 227, 0, 0, 250, 1,
182, 86, 68, 86, 94, 85, 192, 168, 0, 15, 11, 0, 67, 0, 58, 92, 64, 0, 69, 32,
0, 36, 58, 92, 64, 0, 1, 17, 119, 18, 192, 168, 0, 15, 129, 97, 134, 34, 48,
57, 48, 57, 0, 16, 199, 12, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; 

static sample_port: &'static [u8] = &[ 69, 32, 0, 64, 31, 195, 0, 0, 45, 1,
165, 159, 129, 97, 134, 34, 192, 168, 0, 15, 3, 3, 197, 89, 0, 0, 0, 0, 69, 0,
0, 36, 58, 108, 64, 0, 6, 17, 114, 34, 192, 168, 0, 15, 129, 97, 134, 34, 48,
57, 48, 57, 0, 16, 199, 12, 1, 2, 3, 4, 5, 6, 7, 8];

fn print_ip<'a>(mut reader: BufReader<'a>) -> BufReader<'a> {
  let version_and_hdrlen = reader.read_byte().unwrap();
  if version_and_hdrlen != 0x45 {
    fail!("{} isn't 0x45. Not IPv4, or header options set", version_and_hdrlen);
  }
  reader.seek(8, std::io::SeekSet).unwrap();
  let ttl = reader.read_byte().unwrap();
  let protocol = reader.read_byte().unwrap();
  if protocol != 1 { println!("{} isn't 1.  Not ICMP", protocol); }
  reader.seek(12, std::io::SeekSet).unwrap();
  let src_ip = reader.read_be_u32().unwrap();
  let dst_ip = reader.read_be_u32().unwrap();
  print!("IP w/ ttl {} (src {}.{}.{}.{}, dst {}.{}.{}.{}) ", ttl,
           src_ip >> 24, src_ip >> 16 & 255, src_ip >> 8 & 255, src_ip & 255,
           dst_ip >> 24, dst_ip >> 16 & 255, dst_ip >> 8 & 255, dst_ip & 255);
  reader
}
  
fn print_icmp(buf: &[u8]) {
  let mut reader = BufReader::new(buf);
  reader = print_ip(reader);
  let (icmp, code, checksum) = (reader.read_byte().unwrap(),
                                reader.read_byte().unwrap(),
                                reader.read_be_u16().unwrap());
  match icmp {
    1 => println!("Ping reply"),
    3 => println!("Destination unreachable"),
    8 => println!("Ping request"),
   11 => { print!("TTL expired: orig ");
           let _unused = reader.read_le_u32();
           print_ip(reader);
           println!("");
         }
    x => println!("Unhandled ICMP {}", x),
  }
}

fn main() {
  print_icmp(sample_ttl);
  print_icmp(sample_port);
}
