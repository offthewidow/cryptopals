const TABLE: &[u8] = b"0123456789abcdef";

fn table_at(i: u8) -> u8 {
  TABLE[i as usize]
}

pub fn encode(buf: &[u8]) -> Vec<u8> {
  let mut v = Vec::with_capacity(buf.len() * 2);

  for byte in buf {
    v.push(table_at(byte >> 4));
    v.push(table_at(byte & 15));
  }

  v
}

fn decode_chunk(chunk: &[u8]) -> u8 {
  fn convert(byte: u8) -> u8 {
    match byte {
      b'0'..=b'9' => byte - b'0',
      b'a'..=b'f' => byte - b'a' + 10,
      b'A'..=b'F' => byte - b'A' + 10,
      _ => panic!("hex: invalid character"),
    }
  }

  convert(chunk[0]) << 4 | convert(chunk[1])
}

pub fn decode(buf: &[u8]) -> Vec<u8> {
  if buf.len() % 2 != 0 {
    panic!("hex: odd length");
  }

  buf
    .chunks(2)
    .map(decode_chunk)
    .collect()
}