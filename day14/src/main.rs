extern crate libutils;
use libutils::{knot_hash::knot_hash, map2d::Map2D};


fn ascii_to_u8 (input : &str) -> &[u8] {
    input.as_bytes()
}

fn count_ones_bits(input:&[u8;16]) -> u32 {
    let mut count = 0;
    for val in input{
        count += u8::count_ones(*val);
    }
    return count;
}

fn expand_bits(number: u8) -> [bool; 8] {
    [number & 1 != 0,
    number & 2 != 0,
    number & 4 != 0,
    number & 8 != 0,
    number & 16 != 0,
    number & 32 != 0,
    number & 64 != 0,
    number & 128 != 0]
}


fn main() {
    let key = "flqrgnkx";
    let mut count = 0;
    for row in 0..128 {
        let input = format!("{}-{}",key, row);
        let input = ascii_to_u8(&input);
        let hash = knot_hash(input);
        count += count_ones_bits(&hash);
    }
    println!("{}", count);
}


