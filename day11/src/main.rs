extern crate libutils;
use libutils::read_file_to_str;

mod hexmovements;

use crate::hexmovements::walk_hex;
use crate::hexmovements::walk_hex_max_distance;
use crate::hexmovements::get_path;
use crate::hexmovements::Position;
use crate::hexmovements::Movement;


fn parse_direction(mov_direction: &str) -> Result<Movement,()> {
    match mov_direction {
        "n" => Ok(Movement::North),
        "s" => Ok(Movement::South),
        "ne" => Ok(Movement::NorthEast),
        "se" => Ok(Movement::SouthEast),
        "nw" => Ok(Movement::NorthWest),
        "sw" => Ok(Movement::SouthWest),
        _ => Err(())
    }
}

fn parse_directions(line: &str) -> Vec<Movement> {
    let mut movs = Vec::new();
    for split in line.split(",") {
        movs.push(parse_direction(split).unwrap());
    }
    let end_pos = walk_hex(Position::origin(), movs.as_slice());
    println!("Final position: {:?}", end_pos);
    let distance = get_path(end_pos);
    println!("Distance: {}", distance);

    let max_distance = walk_hex_max_distance(Position::origin(), movs.as_slice());
    println!("Max distance: {}", max_distance);

    return movs;

}

fn main() {
    let content = read_file_to_str("input.txt").unwrap();
    parse_directions(content.as_str());
}
