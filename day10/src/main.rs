extern crate libutils;
use libutils::knot_hash::{knot_hash, knot_hash_to_str, basic_knot_hash};


fn ascii_to_u8 (input : &str) -> &[u8] {
    input.as_bytes()
}

fn main() {

    let part1 = basic_knot_hash(&[102,255,99,252,200,24,219,57,103,2,226,254,1,0,69,216]);
    println!("{}", (part1[0] as i32) * (part1[1] as i32));

    let input = "102,255,99,252,200,24,219,57,103,2,226,254,1,0,69,216";
    let input = ascii_to_u8(input);
    let result = knot_hash(input);
    println!("{}",knot_hash_to_str(result));
    println!("{:?}", result);
}
