extern crate libutils;
use libutils::*;
use libutils::FileProcessingErr::IoError;
use libutils::FileProcessingErr::ProcessingError;
use std::collections::HashSet;

fn sort_chars(s: &str) -> String {
    let mut v = s.chars().collect::<Vec<char>>();
    v.sort();
    v.iter().collect()
}



fn process_line(line: &String, allow_reversed:bool) -> bool {
    
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut used = HashSet::new();

    for word in &words {
        let word = if allow_reversed {String::from(*word)}
                    else {sort_chars(*word)};
        if used.contains(&word){
            //println!("Invalid : {}", line);
            return false;
        } else {
            used.insert(word);
        }
    }
    //println!("Valid   : {}", line);
    return true;
}

fn main() {
    let fname = "input.txt";
    let mut count= 0i32;
    let result : Result<(), FileProcessingErr<()>> = read_file_foreach_line(&fname, &mut |line: String| {if process_line(&line, true) {count += 1} Ok(())});
    match result {
        Ok(_) => println!("Ok: {}",count ),
        Err(IoError(_)) => println!("Error while reading file"),
        Err(ProcessingError(_)) => println!("Error while processing file")
    }

    let mut count= 0i32;
    let result : Result<(), FileProcessingErr<()>> = read_file_foreach_line(&fname, &mut |line: String| {if process_line(&line, false) {count += 1} Ok(())});
    match result {
        Ok(_) => println!("Ok: {}",count ),
        Err(IoError(_)) => println!("Error while reading file"),
        Err(ProcessingError(_)) => println!("Error while processing file")
    }
}
#[cfg(test)]
mod tests {
    use super::process_line;
    #[test]
    fn test_normal() {
        let s = String::from("aa bb cc dd ee");
        assert!(process_line(&s, true));
        assert!(process_line(&s, false));
    }
    #[test]
    fn test_duplicate() {
        let s = String::from("aa bb cc dd aa");
        assert!(!process_line(&s, true));
        assert!(!process_line(&s, false));
    }
    #[test]
    fn test_substring() {
        let s = String::from("aa bb cc dd aaa");
        assert!(process_line(&s, true));
        assert!(process_line(&s, false));
    }
    #[test]
    fn test_anagrams() {
        let s = String::from("ab bb cc dd ba");
        assert!(process_line(&s, true));
        assert!(!process_line(&s, false));
    }
}