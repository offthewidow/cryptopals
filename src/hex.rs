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

pub fn decode<T: AsRef<[u8]>>(bytes: T) -> Vec<u8> {
  let bytes = bytes.as_ref();

  if bytes.len() % 2 != 0 {
    panic!("hex: odd length");
  }

  bytes
    .chunks(2)
    .map(decode_chunk)
    .collect()
}