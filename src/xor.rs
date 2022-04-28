pub fn xor<T: AsRef<[u8]>>(a: T, b: T) -> Vec<u8> {
  let (a, b) = (a.as_ref(), b.as_ref());

  if a.len() != b.len() {
    panic!("xor: different length");
  }

  a
    .iter()
    .zip(b.iter())
    .map(|(x, y)| x ^ y)
    .collect()
}