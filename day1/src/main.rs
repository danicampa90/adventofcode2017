
extern crate libutils;
use libutils::read_file_to_str;



fn main() {
    let s = read_file_to_str("input.txt").unwrap();
    println!("DupSum: {}",dupsum_str(&s));
    println!("DupSum: {}",dupsum_str_mid(&s));
}


fn dupsum_str(s : &String) -> i32{
    let mut firstchar = s.chars();
    let mut secondchar = s.chars() 
        .skip(1) // advance of 1
        .chain(s.chars()); // wrap around
    dupsum_iterator(&mut firstchar,&mut secondchar)
}
fn dupsum_str_mid(s : &String) -> i32{
    let mut firstchar = s.chars();
    let mut secondchar = s.chars() 
        .skip(s.len()/2) // advance to middle string
        .chain(s.chars()); // wrap around
    dupsum_iterator(&mut firstchar,&mut secondchar)
}


fn dupsum_iterator(c1: &mut Iterator<Item=char>, c2: &mut Iterator<Item=char>) -> i32 {
    let mut result = 0i32;
    for c in c1 {
        if c == c2.next().unwrap_or('-') {
            result += c.to_digit(10).unwrap_or(0) as i32;
        }
    }


    return result;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test(name="Example 1111")]
    fn test_1 () {
        let s = String::from("1111");
        assert_eq!(dupsum_str(&s), 4)
    }

    #[test(name="Example 1234")]
    fn test_2 () {
        let s = String::from("1234");
        assert_eq!(dupsum_str(&s), 0)
    }

    #[test(name="Example 1122")]
    fn test_3 () {
        let s = String::from("1122");
        assert_eq!(dupsum_str(&s), 3)
    }
}
