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


#[cfg(test)]
mod tests {
    use libutils::knot_hash::{self, knot_hash, knot_hash_to_str};

    use crate::ascii_to_u8;

    #[test]
    pub fn test_empty() {
        let value = knot_hash_to_str(knot_hash(ascii_to_u8("")));
        assert_eq!(value,"a2582a3a0e66e6e86e3812dcb672a272");
    }
    #[test]
    pub fn test_aoc2017() {
        let value = knot_hash_to_str(knot_hash(ascii_to_u8("AoC 2017")));
        assert_eq!(value,"33efeb34ea91902bb2f59c9920caa6cd");
    }
    #[test]
    pub fn test_123() {
        let value = knot_hash_to_str(knot_hash(ascii_to_u8("1,2,3")));
        assert_eq!(value,"3efbe78a8d82f29979031a4aa0b16a9d");
    }
    #[test]
    pub fn test_124() {
        let value = knot_hash_to_str(knot_hash(ascii_to_u8("1,2,4")));
        assert_eq!(value,"63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}