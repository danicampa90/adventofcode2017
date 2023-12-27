use std::{fmt::{Display, Debug}, str::FromStr};

use libutils::peekable_string::PeekableString;


#[derive(PartialEq, Eq)]
pub struct Program{
    pub id: char
}

impl Display for Program
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Program").field("id", &self.id).finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramFromStrErr{
    InvalidChar,
    NotASingleChar,
}

impl FromStr for Program {
    type Err = ProgramFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ch = s.chars().collect::<Vec<char>>();
        if ch.len() != 1 {
            return Err(ProgramFromStrErr::NotASingleChar)
        } 
        let ch = ch[0];
        if ch >= 'a' && ch <= 'p' {
            return Ok(Program{id: ch})
        } else {
            return Err(ProgramFromStrErr::InvalidChar)
        }
    }
}

impl Program {
    pub fn new(ch: char) -> Result<Self, ProgramFromStrErr> {
        if ch >= 'a' && ch <= 'p' {
            return Ok(Program{id: ch})
        } else {
            return Err(ProgramFromStrErr::InvalidChar)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::program::{Program, ProgramFromStrErr};

    #[test]
    pub fn test_parse_validcases(){
        assert_eq!(Program::from_str("a").unwrap().id, 'a');
        assert_eq!(Program::from_str("b").unwrap().id, 'b');
        assert_eq!(Program::from_str("c").unwrap().id, 'c');
        assert_eq!(Program::from_str("d").unwrap().id, 'd');
        assert_eq!(Program::from_str("e").unwrap().id, 'e');
        assert_eq!(Program::from_str("f").unwrap().id, 'f');
        assert_eq!(Program::from_str("g").unwrap().id, 'g');
        assert_eq!(Program::from_str("h").unwrap().id, 'h');
        assert_eq!(Program::from_str("i").unwrap().id, 'i');
        assert_eq!(Program::from_str("j").unwrap().id, 'j');
        assert_eq!(Program::from_str("k").unwrap().id, 'k');
        assert_eq!(Program::from_str("l").unwrap().id, 'l');
        assert_eq!(Program::from_str("m").unwrap().id, 'm');
        assert_eq!(Program::from_str("n").unwrap().id, 'n');
        assert_eq!(Program::from_str("o").unwrap().id, 'o');
        assert_eq!(Program::from_str("p").unwrap().id, 'p');
    }

    #[test]
    pub fn test_parse_invalidchar(){
        assert_eq!(Program::from_str("q"), Err(ProgramFromStrErr::InvalidChar));
        assert_eq!(Program::from_str("#"), Err(ProgramFromStrErr::InvalidChar));
        assert_eq!(Program::from_str("!"), Err(ProgramFromStrErr::InvalidChar));
        assert_eq!(Program::from_str("中"), Err(ProgramFromStrErr::InvalidChar));
        
    }
    #[test]
    pub fn test_parse_not_a_single_char(){
        assert_eq!(Program::from_str("test"), Err(ProgramFromStrErr::NotASingleChar));
        assert_eq!(Program::from_str(""), Err(ProgramFromStrErr::NotASingleChar));
        assert_eq!(Program::from_str("中国"), Err(ProgramFromStrErr::NotASingleChar));
        
    }
}