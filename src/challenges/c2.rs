use cryptopals::hex;
use cryptopals::xor::xor;

const INPUT1: &str = "1c0111001f010100061a024b53535009181c";
const INPUT2: &str = "686974207468652062756c6c277320657965";
const OUTPUT: &str = "746865206b696420646f6e277420706c6179";

pub fn run() {
  println!("CHALLENGE 2...");
  assert_eq!(String::from_utf8_lossy(&hex::encode(xor(hex::decode(INPUT1), hex::decode(INPUT2)))), OUTPUT);
  println!("OK.");
}