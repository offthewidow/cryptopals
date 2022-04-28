use cryptopals::hex;
use cryptopals::base64;

const INPUT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const OUTPUT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

pub fn run() {
  println!("CHALLENGE 1...");
  assert_eq!(String::from_utf8_lossy(&base64::encode(&hex::decode(INPUT))), OUTPUT);
  println!("OK.");
}