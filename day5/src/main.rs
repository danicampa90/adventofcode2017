extern crate libutils;
use libutils::read_file_foreach_line;
use libutils::FileProcessingErr;

fn read_program(fname: &str) -> Result<Vec<i32>, FileProcessingErr<String>> {
    let mut program : Vec<i32> = Vec::new();
    read_file_foreach_line(&fname, &mut |line| {
        if let Ok(num) = line.parse() { 
            program.push(num); 
            Ok(())
        } else {Err(String::from("Cannot parse line"))}
    })?;

    
    return Ok(program);
}

fn execute_program_count_steps_inc(program: &mut Vec<i32>) -> i32 {
    // first star
    let mut instruction_idx :i32 = 0;
    let mut steps = 0;

    while instruction_idx >= 0 && instruction_idx < (program.len() as i32) {
        let cell_value : &mut i32 = &mut program[instruction_idx as usize];
        instruction_idx += *cell_value;
        (*cell_value) += 1;
        steps += 1;
    }

    return steps
}

fn execute_program_count_steps_incdec(program: &mut Vec<i32>) -> i32 {
    // second star
    let mut instruction_idx :i32 = 0;
    let mut steps = 0;

    while instruction_idx >= 0 && instruction_idx < (program.len() as i32) {
        let cell_value : &mut i32 = &mut program[instruction_idx as usize];
        instruction_idx += *cell_value;
        if *cell_value >= 3 { // changed here wrt star 1
            (*cell_value) -= 1;
        } else {
            (*cell_value) += 1;
        }
        steps += 1;
    }

    return steps
}
fn print_program(stepid:i32, state: &Vec<i32>) {
    print!("State {}: ", stepid);
    for x in state {
        print!("{},",x );
    }
    println!();
}

fn main() {
    let fname = "input.txt";
    let mut program : Vec<i32> = read_program(fname).unwrap();
    print_program(-1, &program);
    let steps = execute_program_count_steps_inc(&mut program);
    print_program(999999, &program);
    println!("Executed step 1 program in {} steps", steps);

    println!("--------Step 2---------");
    let mut program : Vec<i32> = read_program(fname).unwrap();
    print_program(-1, &program);
    let steps = execute_program_count_steps_incdec(&mut program);
    print_program(999999, &program);
    println!("Executed step 2 program in {} steps", steps);
}



#[cfg(test)]
mod tests {
    use execute_program_count_steps_inc;
    use execute_program_count_steps_incdec;

    #[test]
    fn test_example_step1() {
        let mut data = vec!(0, 3, 0, 1, -3);
        assert_eq!(execute_program_count_steps_inc(&mut data), 5);
    }
    #[test]
    fn test_example_step2() {
        let mut data = vec!(0, 3, 0, 1, -3);
        assert_eq!(execute_program_count_steps_incdec(&mut data), 10);
    }
}
