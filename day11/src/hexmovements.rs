use std;

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    North,
    South,
    SouthEast,
    NorthEast,
    SouthWest,
    NorthWest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position { 
    x: i32,
    y: i32
}

impl Position{
    pub fn origin() -> Position {
        Position::new(0,0)
    }

    pub fn new(x: i32, y:i32) -> Position {
        Position{x:x, y:y}
    }

    pub fn dist_squared(x: i32, y:i32) -> i32 {
        return x*x + y*y;
    }
}

fn move_hex(p:Position, m: Movement) -> Position {
    let odd_row_offset = if p.x % 2 == 0 { 0 } else { -1 };
    match m {
        Movement::North =>     Position{x: p.x, y: p.y + 1},
        Movement::South =>     Position{x: p.x, y: p.y - 1},
        // next ones depend if we are on odd or even number in x axis
        Movement::NorthEast => Position{x: p.x+1, y: p.y + 1 + odd_row_offset},
        Movement::SouthEast => Position{x: p.x+1, y: p.y + odd_row_offset},
        Movement::NorthWest => Position{x: p.x-1, y: p.y + 1 + odd_row_offset},
        Movement::SouthWest => Position{x: p.x-1, y: p.y + odd_row_offset},
    }
}

pub fn walk_hex(start: Position, directions: &[Movement]) -> Position {
    let mut pos = start;
    for dir in directions {
        pos = move_hex(pos, *dir);
    }
    pos
}
pub fn walk_hex_max_distance(start: Position, directions: &[Movement]) -> i32 {
    let mut pos = start;
    let mut max_distance = 0;
    for dir in directions {
        pos = move_hex(pos, *dir);
        let distance = get_path(pos);
        if distance > max_distance { max_distance = distance; }
    }
    max_distance
}


pub fn get_path(mut pos: Position) -> i32 {
    let mut total_count = 0;
    while pos != Position::origin() {
        let decision = decide_movement(pos);
        if decision.is_none() {
            break;
        }
        let (movement, count) = decision.unwrap();
        //println!("From {:?} move {:?} {} times", pos, movement, count);
        total_count += count;
        for _repetiton in 0..count {
            pos = move_hex(pos, movement);
        }
    }
    return total_count;
}

pub fn decide_movement(pos: Position) -> Option<(Movement, i32)> {
    /*! Returns direction and number of steps in that direction that gets closer to the objective
     * Target is the origin (0,0). 
     **/
    if pos.x == 0 { 
        if pos.y == 0 {
            return None;
        }
        // easy case: move up or down directly.
        if pos.y > 0 {
            return Some((Movement::South,  pos.y));
        } else {
            return Some((Movement::North, -pos.y));
        }
    }
    if pos.y == 0 {
        if pos.x < 0 {
            return Some((Movement::NorthEast, 1))
        } else {
            return Some((Movement::NorthWest, 1))
        }
    }

    
    if pos.x > 0 && pos.y > 0 {
        let distance = std::cmp::min(pos.x, pos.y);
        return Some((Movement::SouthWest, distance))
    } else if pos.x < 0 && pos.y > 0 {
        let distance = std::cmp::min(-pos.x, pos.y);
        return Some((Movement::SouthEast, distance))
    } else if pos.x > 0 && pos.y < 0 {
        let distance = std::cmp::min(pos.x, -pos.y);
        return Some((Movement::NorthWest, distance))
    } else if pos.x < 0 && pos.y < 0 {
        let distance = std::cmp::min(-pos.x, -pos.y);
        return Some((Movement::NorthEast, distance))
    } else { unreachable!() };
}



#[cfg(test)]
mod tests {
    use super::Movement::*;
    use super::walk_hex;
    use super::Position;

    #[test]
    fn test_movement_around() {
        let origin = Position{ x: 0, y:0 };
        let walk = [North, NorthEast, SouthEast, South, NorthWest, SouthWest];
        assert_eq!(walk_hex(origin, &walk), origin);
    }
    #[test] 
    fn test_movement_neneswsw() {
        let origin = Position{ x: 0, y:0 };
        let walk = [NorthEast, NorthEast, SouthWest, SouthWest];
        assert_eq!(walk_hex(origin, &walk), origin);
    }
    #[test] 
    fn test_movement_neness() {
        let origin = Position{ x: 0, y:0 };
        let walk = [NorthEast, NorthEast, South, South];
        assert_eq!(walk_hex(origin, &walk), Position{ x: 2, y:-1 });
    }

}