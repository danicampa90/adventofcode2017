extern crate libutils;
use libutils::read_file_to_str;

use std::rc::Rc;
use std::cell::RefCell;

type ParsedStructRef<'a> = Rc<RefCell<ParsedStruct<'a>>>;
#[derive(Debug)]
enum ParsedStruct<'a>{
    Group(Vec<ParsedStructRef<'a>>),
    Chars(&'a str),
    Garbage(&'a str)
}

fn parse_garbage(content: &str) -> (usize, ParsedStructRef) {
    println!("  parse garbage! {}",content);
    let mut iterator = content.chars();
    let mut count = 0;
    loop {
        count+=1;
        match iterator.next() {
            Some('>') => break,
            Some('!') => { iterator.next().unwrap(); count+=1;}, // skip
            None=> panic!("EOF while parsing garbage"),
            _ => () // ignore everything else
        }
    }
    println!("  parse garbage {} finished",&content[0..count]);
    let garbage = &content[0..count-1];
    return (count, Rc::from(RefCell::from(ParsedStruct::Garbage(garbage))));
}

fn parse_group(content: &str) -> (usize, ParsedStructRef) {
    println!("  parse group! {}",content);
    let mut inner = vec!();
    let mut count = 0;
    let mut iterator = content.chars();
    let mut beginning = true;
    loop {
        match iterator.next() {
            Some('}') => { count += 1; break},
            Some(',') if !beginning => {
                count += 1; 
                let current_place = &content[count..];
                let (inner_count, inner_content) = parse(current_place);
                println!("       - {:?} was using {} chars",&inner_content, inner_count);
                inner.push(inner_content);
                count += inner_count; // inner count
                iterator = content[count..].chars(); // iterator
            }
            Some(_) if beginning => { 
                let current_place = &content[count..];
                let (inner_count, inner_content) = parse(current_place);
                println!("       - {:?} was using {} chars",&inner_content, inner_count);
                inner.push(inner_content);
                count += inner_count; // inner count
                iterator = content[count..].chars(); // iterator
                beginning = false;
            }
            Some(_) => panic!("Unexpected token in group. Was expecting , or {."),
            None => panic!("EOF while parsing group!")
        }
    }
    println!("  parse group {} finished",&content[0..count]);
    return (count, Rc::from(RefCell::from(ParsedStruct::Group(inner))));
}

fn parse_text(content: &str) -> (usize, ParsedStructRef) {
    // TODO: quite inefficient
    return (1, Rc::from(RefCell::from(ParsedStruct::Chars(&content[0..1]))));
}


// returns end of the string ptr and the parsed data
fn parse(content: &str) -> (usize, ParsedStructRef) {
    println!("Parse {}",content);
    let (parsed_size, parsed_data) = match content.chars().next() {
        Some('<') => parse_garbage(&content[1..]),
        Some('!') => parse(&content[2..]),
        Some('{') => parse_group(&content[1..]),
        Some(_) => parse_text(&content),
        None => panic!("Unexpected EOF while looking for next token")
    };

    return (parsed_size+1, parsed_data);
}

fn calculate_score(structure : &ParsedStructRef) -> i32 {
    0
}


fn get_score (content: &str) -> i32 {
    let (_size, data) = parse(content);
    println!("{:?}",data );
    calculate_score(&data)
}

fn main() {
    let content = read_file_to_str("input.txt").unwrap();
    let score = get_score(&content);
    println!("score {}", score);
}

#[cfg(test)]
mod test{
    use get_score;

    #[test]
    fn test_groups1() {
        assert_eq!(get_score("{}"), 1);
    }

    #[test]
    fn test_groups2() {
        assert_eq!(get_score("{{}}"), 3);
    }

    #[test]
    fn test_groups3() {
        assert_eq!(get_score("{{{}}}"), 6);
    }

    #[test]
    fn test_groups4() {
        assert_eq!(get_score("{{},{}}"), 5);
    }

    #[test]
    fn test_groups5() {
        assert_eq!(get_score("{{{},{},{{}}}}"), 16);
    }

    

    #[test]
    fn test_garbage1() {
        assert_eq!(get_score("{<a>,<a>,<a>,<a>}"), 1);
    }

    #[test]
    fn test_garbage2() {
        assert_eq!(get_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    }
    #[test]
    fn test_garbage3() {
        assert_eq!(get_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    }
    #[test]
    fn test_garbage4() {
        assert_eq!(get_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}