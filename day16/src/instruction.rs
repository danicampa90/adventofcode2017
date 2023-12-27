use std::str::FromStr;

use libutils::peekable_string::PeekableString;

use crate::{Program, program::ProgramFromStrErr};

pub enum Instruction {
    Spin(i32),
    Exchange(i32, i32),
    Partner(Program, Program)
}

pub enum InstructionFromStrErr {
    EmptyInstructionError,
    InvalidInstructionError,
    ProgramParseError(ProgramFromStrErr),
    NumberParseError,
    UnexpectedTokenError(char)
    
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
            s.push(str.pop());
        }
        return i32::from_str(&s).map_err(|_|InstructionFromStrErr::NumberParseError);
    }

    fn parse_separator(str: &mut PeekableString) -> Result<(), InstructionFromStrErr> {
        if str.peek() == Some('/') {
            return Ok(())
        } else {
            return Err(InstructionFromStrErr::UnexpectedTokenError(str.pop()))
        }
    }

}

