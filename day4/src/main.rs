extern crate libutils;
use libutils::*;
use std::borrow::Borrow;
use std::collections::HashSet;

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}



fn process_line(line: &String) -> bool {
    
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut used = HashSet::new();

    for word in &words {
        let word = String::from(*word);
        if used.contains(&word){
            println!("Invalid : {}", line);
            return false;
        } else {
            used.insert(word);
        }
    }
/*
    for word in words.into_iter().map(reverse) {
        let word = String::from(word);
        if used.contains(&word){
            println!("Invalid : {}", line);
            return false;
        } else {
            used.insert(word);
        }
    }*/
    println!("Valid   : {}", line);
    return true;
}

fn main() {
    let fname = "input.txt";
    let mut count= 0i32;
    let result = read_file_foreach_line(&fname, &mut |line: String| {if process_line(&line) {count += 1} Ok(())});
    match result {
        Ok(_) => println!("Ok: {}",count ),
        Err(_) => println!("Error while reading file")
    }
}
#[cfg(test)]
mod tests {
    use super::process_line;
    #[test(normal)]
    fn test_1() {
        let s = String::from("aa bb cc dd ee");
        assert!(process_line(&s));
    }
    #[test(duplicate)]
    fn test_2() {
        let s = String::from("aa bb cc dd aa");
        assert!(!process_line(&s));
    }
    #[test(substring)]
    fn test_3() {
        let s = String::from("aa bb cc dd aaa");
        assert!(process_line(&s));
    }
    #[test(palindromes)]
    fn test_4_palindromes() {
        let s = String::from("ab bb cc dd ba");
        assert!(process_line(&s));
    }
}