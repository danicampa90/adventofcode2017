use std::str::FromStr;

use libutils::peekable_string::PeekableString;

use crate::{Program, program::ProgramFromStrErr};

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    Spin(i32),
    Exchange(i32, i32),
    Partner(Program, Program)
}

#[derive(PartialEq, Eq, Debug)]
pub enum InstructionFromStrErr {
    EmptyInstructionError,
    InvalidInstructionError,
    ProgramParseError(ProgramFromStrErr),
    NumberParseError,
    UnexpectedTokenError(char),
    EofError
    
}

impl FromStr for Instruction{
    type Err = InstructionFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut peekable_str = PeekableString::new(s);
        Instruction::parse(&mut peekable_str)
    }
}

impl Instruction {

    pub fn parse(peekable_str: &mut PeekableString) -> Result<Instruction, InstructionFromStrErr> {

        match peekable_str.pop() {
            Some('s') => Instruction::parse_spin(peekable_str),
            Some('x') => Instruction::parse_exchange(peekable_str),
            Some('p') => Instruction::parse_partner(peekable_str),
            None => Err(InstructionFromStrErr::EmptyInstructionError),
            Some(_) => Err(InstructionFromStrErr::InvalidInstructionError)
        }
    }

    fn parse_exchange(str: &mut PeekableString) -> Result<Instruction, InstructionFromStrErr>{
        let num1 = Instruction::parse_number(str)?;
        Instruction::parse_separator(str)?;
        let num2 = Instruction::parse_number(str)?;
        return Ok(Instruction::Exchange(num1, num2));
    }

    fn parse_spin(str: &mut PeekableString) -> Result<Instruction, InstructionFromStrErr>{
        let num1 = Instruction::parse_number(str)?;
        return Ok(Instruction::Spin(num1));
    }

    fn parse_partner(str: &mut PeekableString) -> Result<Instruction, InstructionFromStrErr>{

        let ch1 = str.pop()
            .ok_or(InstructionFromStrErr::ProgramParseError(ProgramFromStrErr::NotASingleChar))?;
        Instruction::parse_separator(str)?;
        let ch2 = str.pop()
            .ok_or(InstructionFromStrErr::ProgramParseError(ProgramFromStrErr::NotASingleChar))?;

        let prog1 = Program::new(ch1)
            .map_err(|e| InstructionFromStrErr::ProgramParseError(e))?;
        let prog2 = Program::new(ch2)
            .map_err(|e| InstructionFromStrErr::ProgramParseError(e))?;

        return Ok(Instruction::Partner(prog1, prog2));
    }

    fn parse_number(str: &mut PeekableString) -> Result<i32, InstructionFromStrErr> {
        let mut s = String::new();
        while let Some(ch) = str.peek() {
            if !ch.is_numeric() {
                break;
            }
            s.push(str.pop().unwrap());
        }
        return i32::from_str(&s).map_err(|_|InstructionFromStrErr::NumberParseError);
    }

    fn parse_separator(str: &mut PeekableString) -> Result<(), InstructionFromStrErr> {
        return if str.peek() == Some('/') {
            str.pop();
            Ok(())
        } else {
            match str.pop() {
                Some(ch) => Err(InstructionFromStrErr::UnexpectedTokenError(ch)),
                None => Err(InstructionFromStrErr::EofError)
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use std::str::FromStr;

    use crate::{instruction::{Instruction, InstructionFromStrErr}, program::{Program, ProgramFromStrErr}};


    #[test]
    pub fn test_parse_spin() {


        assert_eq!(Ok(Instruction::Spin(1)), Instruction::from_str("s1"));
        assert_eq!(Ok(Instruction::Spin(11)), Instruction::from_str("s11"));

        assert_eq!(Ok(Instruction::Spin(1)), Instruction::from_str("s1,"));
        assert_eq!(Ok(Instruction::Spin(13)), Instruction::from_str("s13,"));

        assert_eq!(Err(InstructionFromStrErr::NumberParseError), Instruction::from_str("s"));
    }

    #[test]
    pub fn test_parse_exchange() {
        assert_eq!(Ok(Instruction::Exchange(1,2)), Instruction::from_str("x1/2"));
        assert_eq!(Ok(Instruction::Exchange(1,22)), Instruction::from_str("x1/22"));
        assert_eq!(Ok(Instruction::Exchange(11,2)), Instruction::from_str("x11/2"));
        assert_eq!(Ok(Instruction::Exchange(11,12)), Instruction::from_str("x11/12"));

        assert_eq!(Ok(Instruction::Exchange(1,22)), Instruction::from_str("x1/22,"));

        assert_eq!(Err(InstructionFromStrErr::NumberParseError), Instruction::from_str("x1/"));
        assert_eq!(Err(InstructionFromStrErr::EofError), Instruction::from_str("x1"));
        assert_eq!(Err(InstructionFromStrErr::NumberParseError), Instruction::from_str("x"));
    }
    
    #[test]
    pub fn test_parse_partner() {
        assert_eq!(Ok(Instruction::Partner(Program::new('a').unwrap(), Program::new('b').unwrap())),
        Instruction::from_str("pa/b"));
        assert_eq!(Err(InstructionFromStrErr::ProgramParseError(ProgramFromStrErr::InvalidChar)),
        Instruction::from_str("pa/z"));
        assert_eq!(Err(InstructionFromStrErr::ProgramParseError(ProgramFromStrErr::NotASingleChar)),
        Instruction::from_str("pa/"));
        assert_eq!(Err(InstructionFromStrErr::EofError), Instruction::from_str("pa"));
        assert_eq!(Err(InstructionFromStrErr::ProgramParseError(ProgramFromStrErr::NotASingleChar)),
        Instruction::from_str("p"));
    }
}