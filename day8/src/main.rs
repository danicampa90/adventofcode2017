extern crate lalrpop_util;
extern crate libutils;

use std::collections::HashMap;

mod grammar;
mod ast;
use ast::Instruction;
use ast::Operation;
use ast::Condition;


struct State {
    registers: HashMap<String, i32>,
    max_value: i32
}

impl State {
    fn new() -> State{
        State{registers: HashMap::new(), max_value: -99999999}
    }
    fn get_value(&mut self, name: &String) -> i32 {
        match self.registers.get(name) {
            Some(val) => *val,
            None => 0
        }
    }
    fn set_value(&mut self, name: String, val:i32) {
        self.registers.insert(name, val);
        if self.max_value < val {
            self.max_value = val;
        }
    }
    fn get_max(&self) -> i32 { self.max_value }
    fn dump(&self) {
        for (key,val) in self.registers.iter() {
            println!(" - {} = {}", key, val);
        }

    }
}

fn eval_cond_op(value1: i32, op: Condition, value2: i32) -> bool {
    match op {
        Condition::GT => value1 > value2,
        Condition::GE => value1 >= value2,
        Condition::EQ => value1 == value2,
        Condition::NE => value1 != value2,
        Condition::LE => value1 <= value2,
        Condition::LT => value1 < value2,
    }
}

fn execute_action(state: &mut State, register: String, op: Operation, value: i32) {
    let old_value = state.get_value(&register);
    let to_add = match op {
        Operation::Inc => value,
        Operation::Dec => -value
    };

    state.set_value(register, old_value + to_add)
}


fn execute (state: &mut State, instr: Option<Instruction>) -> Result<(), String> {
    if instr.is_none() { return Err(String::from("No instructions given")); }
    let instr = instr.unwrap();

    let value = state.get_value(&instr.cond_left);
    if eval_cond_op(value, instr.cond_op, instr.cond_right) {
        // it's true! execute action
        execute_action(state, instr.target_reg, instr.target_op, instr.target_value);
    } else {
    }

    Ok(())
}

fn parse(line: &String) -> Option<Instruction> {
    match grammar::parse_Instr(line) {
        Ok(i) => Some(i),
        Err(_) => { println!("Cannot parse {}", line); None }
    }
}

fn main() {
    let fname = "input.txt";
    let mut state = State::new();
    libutils::read_file_foreach_line(fname,  
        &mut |line: String| execute(&mut state, parse(&line))
    ).unwrap();

    state.dump();

    println!("Max value {}", state.get_max());
/*
    let parsed = grammar::parse_Instr("bxa inc 123 if aaa > -1213");
    match parsed {
        Ok(result) => println!("Ok: {}", result),
        Err(_) => println!("Some error occurred")
    } */
}
