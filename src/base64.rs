const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn table_at(i: u32) -> u8 {
  TABLE[(i & 63) as usize]
}

fn encode_chunk(chunk: &[u8]) -> [u8; 4] {
  let chunk = ((chunk[0] as u32) << 16) + ((chunk[1] as u32) << 8) + (chunk[2] as u32);
  [table_at(chunk >> 18), table_at(chunk >> 12), table_at(chunk >> 6), table_at(chunk)]
}

pub fn encode(buf: &[u8]) -> Vec<u8> {
  buf
    .chunks(3)
    .map(encode_chunk)
    .flatten()
    .collect()
}