pub fn fixed(a: &[u8], b: &[u8]) -> Vec<u8> {
  if a.len() != b.len() {
    panic!("xor: different length");
  }

  a
    .iter()
    .zip(b.iter())
    .map(|(x, y)| x ^ y)
    .collect()
}