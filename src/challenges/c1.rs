use crate::{base64, hex};

#[test]
fn run() {
  let input = hex::decode(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  let output = base64::encode(&input);

  assert_eq!(String::from_utf8_lossy(&output), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}