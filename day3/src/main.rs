// Part 2 is incomplete!
use std::collections::HashMap;

fn manhattan(x: i32, y:i32) -> i32 {
    return if x<0 {-x} else {x} + if y<0 {-y} else {y} 
}
fn find_distance(target: i32) -> i32 {
    let (x,y) = find_coords(target);
    manhattan(x,y)
}
/// Given an index i it returns the coordinates of the i-th cell in the spiral.
/// It's not fully optimized (especially for the while/for loops that go around the spiral).
/// but it goes "out" quite quickly because it exploits the numeric series of the rings.
/// Note: doesn't work with target = 0
fn find_coords(target: i32) -> (i32,i32) {
    let target = target - 1; // we start from 0-based indexes

    // find the "ring number" (the index of a square 
    // expanding from the center) of the number by looking
    // at the bottom right corner (which always contains
    // the maximum number in that ring).
    // This corner also follows some arithmetic series (0, 8, 24, 48)
    // for each number i, we have (0 + 8 + 16 + 24 + 32 + 48...) i times

    let mut ring_id = 0;
    let mut current_corner = 0;
    while target > current_corner {
        ring_id+=1;
        current_corner += 8*ring_id;
        //println!("Corner  , {} - {},{}", current_corner, ring_id, ring_id);
    }


    let (mut x, mut y) = (ring_id, ring_id);
    let mut current = current_corner;

    if target == current_corner {
        return (x,y);
    }
    
    
    // go left
    for _ in 0..(2*ring_id) {
        x-= 1;
        current -=1;
        if current == target{
            return (x,y);
        }
        //println!("Left    , {} - {},{}", current, x,y);
    }
    // jump up left
    for _ in 0..(2*ring_id) {
        y-= 1;
        current -= 1;
        if current == target {
            return (x,y);
        }
        //println!("Up      , {} - {},{}", current, x,y);
    }

    // go left
    for _ in 0..(2*ring_id) {
        x+= 1;
        current -=1;
        if current == target{
            return (x,y);
        }
        //println!("Right   , {} - {},{}", current, x,y);
    }
    // jump up left
    for _ in 0..(2*ring_id) {
        y+= 1;
        current -= 1;
        if current == target {
            return (x,y);
        }
        //println!("Down    , {} - {},{}", current, x,y);
    }
    panic!("Bug: reached end of spiral");

}

// ///////////////// Second star section ///////////////////
// Following functions are the implementation of the second star problem.
// It also uses the function above (find_coords) to go around the spiral

/// Type of the map
type Map = HashMap<(i32,i32), i32>;

/// Sums the neighbors on the map around the specified point
fn sum_neighbors(map: &Map, x:i32, y: i32) -> i32 {
    let mut sum = 0;
    for x_offset in -1..2 { // 2 excluded: [-1, 0, 1]
        for y_offset in -1..2 { // 2 excluded: [-1, 0, 1]
            sum += map.get(&(x_offset+x, y_offset+y)).unwrap_or(&0);
        }
    }
    return sum;
}

/// Solves the second star problem
/// Note that it's very inefficient at the moment. It calls every
/// iteration the function find_coords to find the next coord to fill.
/// This could be optimized by going around the spiral in an iterative way.
fn star2(input: i32) -> i32{
    let mut map: Map = HashMap::new();
    map.insert((0,0), 1);
    let mut index = 1;
    loop {
        let (x,y) = find_coords(index);
        let sum = sum_neighbors(&map, x,y);
        //println!("Insert {} in {},{}", sum, x, y);
        map.insert((x,y), sum);
        if sum > input {
            return sum;
        }
        index += 1;
    }
}

/// Main
fn main() {
    let distance = find_distance(347991);
    println!("Distance: {}", distance);
    let second_star = star2(347991);
    println!("Next value larger than 347991: {}", second_star );
}

#[cfg(test)]
mod tests {
    use super::find_distance;

    #[test(t1)]
    fn test_1() {
        assert_eq!(find_distance(1), 0);
    }

    #[test(t12)]
    fn test_12() {
        assert_eq!(find_distance(12), 3);
    }

    #[test(t23)]
    fn test_23() {
        assert_eq!(find_distance(23), 2);
    }

    #[test(t1024)]
    fn test_1024() {
        assert_eq!(find_distance(1024), 31);
    }

}