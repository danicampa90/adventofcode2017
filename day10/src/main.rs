extern crate libutils;
mod knot_hash;

use knot_hash::knot_hash;


fn ascii_to_u8 (input : &str) -> &[u8] {
    input.as_bytes()
}

fn main() {
    let input = "102,255,99,252,200,24,219,57,103,2,226,254,1,0,69,216";
    let input = ascii_to_u8(input);
    let result = knot_hash(input);
    for byte in result.iter() {
        if *byte < 0x10 {
            print!("0"); // hack 
        }
        print!("{:x}", byte);    
    }
    println!();
    println!("{:?}", result);
}
