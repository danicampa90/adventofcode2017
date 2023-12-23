extern crate libutils;
use libutils::{knot_hash::{knot_hash, knot_hash_to_str}, map2d::Map2D};


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
    [number & 128 != 0,
    number & 64 != 0,
    number & 32 != 0,
    number & 16 != 0,
    number & 8 != 0,
    number & 4 != 0,
    number & 2 != 0,
    number & 1 != 0]
}

fn hash_bytes_to_bits(hash: [u8; 16]) -> Vec<bool> {
    let mut bits = Vec::new();
    for byte in hash {
        for bit in expand_bits(byte) {
            bits.push(bit);
        }
    }
    return bits

}


fn print_regions(map: &Map2D<i32>) {
    for y in 0..8 {   
        for x in 0..8{
            let region = map.get_value(x, y);
            if region < 0 {
                print!("....|");
            }
            else {
                print!("{:04}|", region)
            }
        }
        println!();
    }
}

fn print_map(map: &Map2D<bool>) {
    for y in 0..map.size_y() {   
        for x in 0..map.size_x(){
            let region = map.get_value_usize(x, y);
            if region {
                print!("#");
            }
            else {
                print!(".")
            }
        }
        println!();
    }
}

fn main() {
    let key = "hwlqcszp";
    let mut map = Map2D::new(false, 16*8);

    let mut count = 0;
    for row in 0..128 {
        let input = format!("{}-{}",key, row);
        let input = ascii_to_u8(&input);
        let hash = knot_hash(input);
        let hash_bits = hash_bytes_to_bits(hash);
        
        for col in 0..hash_bits.len() {
            map.set_value(col as i32, row, hash_bits[col])
        }
    }

    print_map(&map);
    let count = map.fold(0, |cnt, b, _,_| if *b {cnt +1} else {cnt});
    println!("count of bits: {}", count);

    let regions = map.regions_with_filter(|b|*b, |b1, b2| b1==b2);
    print_regions(&regions);
    let count_of_regions = regions.fold(0, |val, region_id, _,_| if val<*region_id {*region_id} else {val});
    println!("count of regions:{}", count_of_regions+1);

}


