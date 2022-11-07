use crate::{hex, xor};

#[test]
fn run() {
  let input_1 = hex::decode(b"1c0111001f010100061a024b53535009181c");
  let input_2 = hex::decode(b"686974207468652062756c6c277320657965");
  let output = hex::encode(&xor::fixed(&input_1, &input_2));

  assert_eq!(String::from_utf8_lossy(&output), "746865206b696420646f6e277420706c6179");
}