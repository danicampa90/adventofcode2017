extern crate libutils;
use libutils::read_file_to_str;

use std::rc::Rc;
use std::cell::RefCell;

type ParsedStructRef<'a> = Rc<RefCell<ParsedStruct<'a>>>;
#[derive(Debug)]
enum ParsedStruct<'a>{
    Group(Vec<ParsedStructRef<'a>>),
    Chars(&'a str),
    Garbage(&'a str, i32)
}

fn parse_garbage(content: &str) -> (usize, ParsedStructRef) {
    //println!("  parse garbage! {}",content);
    let mut iterator = content.chars();
    let mut count = 0;
    let mut char_count = 0;
    loop {
        count += 1;
        match iterator.next() {
            Some('>') => break,
            Some('!') => { iterator.next().unwrap(); count+=1; char_count-=1;}, // skip
            None=> panic!("EOF while parsing garbage"),
            _ => () // ignore everything else
        }
        char_count +=1;
    }
    //println!("  parse garbage {} finished",&content[0..count]);
    let garbage = &content[0..count-1];
    return (count, Rc::from(RefCell::from(ParsedStruct::Garbage(garbage, char_count))));
}

fn parse_group(content: &str) -> (usize, ParsedStructRef) {
    //println!("  parse group! {}",content);
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
                //println!("       - {:?} was using {} chars",&inner_content, inner_count);
                inner.push(inner_content);
                count += inner_count; // inner count
                iterator = content[count..].chars(); // iterator
            }
            Some(_) if beginning => { 
                let current_place = &content[count..];
                let (inner_count, inner_content) = parse(current_place);
                //println!("       - {:?} was using {} chars",&inner_content, inner_count);
                inner.push(inner_content);
                count += inner_count; // inner count
                iterator = content[count..].chars(); // iterator
                beginning = false;
            }
            Some(_) => panic!("{}", "Unexpected token in group. Was expecting , or {."),
            None => panic!("EOF while parsing group!")
        }
    }
    //println!("  parse group {} finished",&content[0..count]);
    return (count, Rc::from(RefCell::from(ParsedStruct::Group(inner))));
}

fn parse_text(content: &str) -> (usize, ParsedStructRef) {
    // TODO: quite inefficient because it accepts only a letter
    let mut count = None;
    for (index, character) in content.char_indices() {
        if !character.is_alphabetic() {
            count = Some(index);
            break;
        }
    }

    let count = match count {
        Some(count) => count,
        None => content.len(),
    };

    return (count, Rc::from(RefCell::from(ParsedStruct::Chars(&content[0..count]))));
}


// returns end of the string ptr and the parsed data
fn parse(content: &str) -> (usize, ParsedStructRef) {
    //println!("Parse {}",content);
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
    calculate_score_rec(1, structure)
}

fn calculate_score_rec(depth: i32, structure : &ParsedStructRef) -> i32 {
    let node : &ParsedStruct = &structure.as_ref().borrow();
    match node {
        &ParsedStruct::Chars(_) => 0,
        &ParsedStruct::Garbage(_, _) => 0,
        &ParsedStruct::Group(ref inner) => {
            let mut score = depth;
            for r in inner {
                score += calculate_score_rec(depth+1, r);
            }
            score
        }
    }
}

fn count_garbage(structure : &ParsedStructRef) -> i32 {
    count_garbage_rec(structure)
}

fn count_garbage_rec(structure : &ParsedStructRef) -> i32 {
    let node : &ParsedStruct = &structure.as_ref().borrow();
    match node {
        &ParsedStruct::Chars(_) => 0,
        &ParsedStruct::Garbage(_, garbage_char_count) => garbage_char_count as i32,
        &ParsedStruct::Group(ref inner) =>  {
            let mut score = 0;
            for r in inner {
                score += count_garbage_rec(r);
            }
            score
        }
    }
}


fn get_score (content: &str) -> i32 {
    let (_size, data) = parse(content);
    //println!("{:?}",data );
    calculate_score(&data)
}

fn get_garbage_count (content: &str) -> i32 {
    let (_size, data) = parse(content);
    //println!("{:?}",data );
    count_garbage(&data)
}


fn main() {
    let content = read_file_to_str("input.txt").unwrap();
    let score = get_score(&content);
    println!("score {}", score);
    let garbage = get_garbage_count(&content);
    println!("garbage {}", garbage);
}

#[cfg(test)]
mod test{
    use crate::get_score;
    use crate::get_garbage_count;

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


    #[test]
    fn test_garbage_count_1() {
        assert_eq!(get_garbage_count("<>"), 0);
    }
    #[test]
    fn test_garbage_count_2() {
        assert_eq!(get_garbage_count("<random characters>"), 17);
    }
    #[test]
    fn test_garbage_count_3() {
        assert_eq!(get_garbage_count("<<<<>"), 3);
    }
    #[test]
    fn test_garbage_count_4() {
        assert_eq!(get_garbage_count("<{!>}>"), 2);
    }
    #[test]
    fn test_garbage_count_5() {
        assert_eq!(get_garbage_count("<!!>"), 0);
    }
    #[test]
    fn test_garbage_count_6() {
        assert_eq!(get_garbage_count("<!!!>>"), 0);
    }
    #[test]
    fn test_garbage_count_7() {
        assert_eq!(get_garbage_count("<{o\"i!a,<{i<a>"), 10);
    }
}