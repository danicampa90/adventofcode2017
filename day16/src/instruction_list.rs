use libutils::peekable_string::PeekableString;

use crate::instruction::{Instruction, InstructionFromStrErr};


pub struct InstructionList {
    instructions: Vec<Instruction>
}

impl InstructionList {
    pub fn new(instructions: Vec<Instruction>) -> InstructionList {
        InstructionList{instructions}
    }
    pub fn list(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn parse(peekable_str: &mut PeekableString) -> Result<InstructionList, InstructionFromStrErr> {
        let instruction = Instruction::parse(peekable_str)?;
        let mut list = Vec::new();
        list.push(instruction);

        while Self::parse_comma(peekable_str).is_ok() {
            let instr = Instruction::parse(peekable_str)?;
            list.push(instr);
        }

        return Ok(InstructionList::new(list))
    }

    fn parse_comma(peekable_str: &mut PeekableString) -> Result<(), InstructionFromStrErr> {
        match peekable_str.pop() {
            Some(',') => Ok(()),
            Some(ch) => Err(InstructionFromStrErr::UnexpectedTokenError(ch)),
            None => Err(InstructionFromStrErr::EofError)
        }
    }
}