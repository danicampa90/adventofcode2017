// Part 2 is incomplete!


fn manhattan(x: i32, y:i32) -> i32 {
    return if x<0 {-x} else {x} + if y<0 {-y} else {y} 
}
fn find_distance(target: i32) -> i32 {
    let (x,y) = find_coords(target);
    manhattan(x,y)
}

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


fn main() {
    let distance = find_distance(347991);
    println!("{}", distance);
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