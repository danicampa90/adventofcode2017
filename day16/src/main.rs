mod program;
mod instruction;
mod instruction_list;

use libutils::peekable_string::PeekableString;
use program::Program;
use instruction_list::InstructionList;

fn main() {
    let input = libutils::read_file_to_str("input.txt").unwrap();
    let mut input = PeekableString::new(&input);
    let list = InstructionList::parse(&mut input).unwrap();
    println!("Parsed {} instructions", list.list().len());
}
