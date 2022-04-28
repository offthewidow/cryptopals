const TABLE: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

fn table_at(i: u32) -> u8 {
  TABLE[(i & 63) as usize]
}

fn encode_chunk(chunk: &[u8]) -> [u8; 4] {
  let chunk = ((chunk[0] as u32) << 16) + ((chunk[1] as u32) << 8) + (chunk[2] as u32);
  [ table_at(chunk >> 18), table_at(chunk >> 12), table_at(chunk >> 6), table_at(chunk) ]
}

pub fn encode<T: AsRef<[u8]>>(bytes: T) -> Vec<u8> {
  bytes
    .as_ref()
    .chunks(3)
    .map(encode_chunk)
    .flatten()
    .collect()
}