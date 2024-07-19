use std::mem;

use crate::{instruction_list::InstructionList, program::{self, Program}};


pub struct Executor {
    programs: Vec<Program>
}

impl Executor{
    pub fn new(size: u32) -> Executor {
        let mut programs = Vec::new();
        let a_value = 'a' as u32;
        
        for i in 0..size {
            programs.push(Program::new(char::from_u32(i+a_value).unwrap()).unwrap())
        }

        Executor{programs}
    }

    pub fn run(&mut self, instructions: &InstructionList) {
        for instr in instructions.list() {
            match instr {
                crate::instruction::Instruction::Spin(num) => {
                    let (start_arr,end_arr) = self.programs.split_at(self.programs.len() - *num as usize);
                    let mut new_arr = Vec::from(end_arr);
                    new_arr.extend_from_slice(start_arr);
                    mem::swap(&mut new_arr, &mut self.programs);

                },
                crate::instruction::Instruction::Exchange(a, b) => {
                    self.programs.swap(*a as usize, *b as usize);
                },
                crate::instruction::Instruction::Partner(a, b) => {
                    let indexes : Vec<usize> = self.programs.iter().enumerate().filter(|(idx,item) | *item == a || *item == b).map(|item| item.0).collect();
                    if indexes.len() != 2 {
                        panic!("Found {} entries matching letter {} and {}", indexes.len(), a.id, b.id);
                    }
                    self.programs.swap(indexes[0], indexes[1])
                },
            }
        }
    }

    pub fn order_string(&self) -> String {
        let mut out = String::new();
        for prog in self.programs.iter() {
            out.push(prog.id);
        }
        return out;
    }
}


#[cfg(test)]
mod tests{
    use crate::instruction_list::InstructionList;

    use super::Executor;

    #[test]
    pub fn test_spin() {
        let mut exec = Executor::new(5);
        let mut instruction = "s1".into();
        exec.run(&InstructionList::parse(&mut instruction).unwrap());
        assert_eq!("eabcd", exec.order_string());
    }

    #[test]
    pub fn test_exchange() {
        let mut exec = Executor::new(5);
        let mut instruction = "s1,x3/4".into();
        exec.run(&InstructionList::parse(&mut instruction).unwrap());
        assert_eq!("eabdc", exec.order_string());
    }

    #[test]
    pub fn test_partner() {
        let mut exec = Executor::new(5);
        let mut instruction = "s1,x3/4,pe/b".into();
        exec.run(&InstructionList::parse(&mut instruction).unwrap());
        assert_eq!("baedc", exec.order_string());
    }
}