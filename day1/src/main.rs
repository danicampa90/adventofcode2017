use std::fs::File;
use std::io::Read;
use std::path::Path;


#[cfg(test)]
#[test(name="Example 1111")]
fn test_1 () {
    assert_eq!(dupsum_str(String::from("1111")), 4)
}
#[cfg(test)]
#[test(name="Example 1234")]
fn test_2 () {
    assert_eq!(dupsum_str(String::from("1234")), 0)
}
#[cfg(test)]
#[test(name="Example 1122")]
fn test_3 () {
    assert_eq!(dupsum_str(String::from("1122")), 3)
}


fn main() {
    let s = read_file("input.txt");
    println!("DupSum: {}",dupsum_str(&s));
    println!("DupSum: {}",dupsum_str_mid(&s));
}

fn read_file(fname: &str) -> String {
    println!("Loading {0}", fname);
    let mut file : File = File::open(Path::new(fname)).unwrap();
    let mut s = String::new();
    let size = file.read_to_string(&mut s).unwrap();
    println!("Read {} bytes",size );
    return s
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
