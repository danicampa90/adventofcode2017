
use std::collections::HashSet;

fn get_candidate_index<T>(input: &Vec<T>) -> Option<usize>
    where T : Ord
{
    let mut max_idx = None;
    let mut max_value = None;
    let mut current_idx = 0;
    for item in input {
        let exchange = match max_value {
            None => true,
            Some(x) if x < item => true,
            _ => false
        };

        if exchange {
            max_idx = Some(current_idx);
            max_value = Some(item);
        };

        current_idx += 1;
    }
    return max_idx;
}


fn redistribute_elements(input: &mut Vec<i32>) -> Result<(), &'static str>
{
    let idx = get_candidate_index(input);
    let idx : usize = if let Some(idx) = idx { idx } else {
        return Err("Cannot find candidate index. Is input empty?");
    };

    // start redistribution process
    let mut to_redist : i32 = input[idx]; // get current bank value
    input[idx] = 0; // set that bank value to 0

    let mut current_idx = idx; // start distribution from next element: the first step in the loop is incrementing this.
    let max_idx = input.len() - 1 ;

    while to_redist > 0 {
        current_idx = if current_idx < max_idx { current_idx + 1 } else { 0 };
        input[current_idx] += 1;
        to_redist -= 1;
    }

    Ok(())
}

fn print_state(stepid:i32, state: &Vec<i32>) {
    print!("State {}: ", stepid);
    for x in state {
        print!("{},",x );
    }
    println!();
}

fn find_loop_in_states(vec: Vec<i32>) -> i32 {
    let mut states = HashSet::new();
    let mut current_state = vec;
    let mut counter = 0;
    loop {
        print_state(counter, &current_state);

        if states.contains(&current_state){
            return counter;
        }
        states.insert(current_state.clone());
        redistribute_elements(&mut current_state).unwrap();
        counter += 1;

    }

}


fn main(){
    let mut input = vec!(2,8,8,5,4,2,3,1,5,5,1,2,15,13,5,14);
    //let input = vec!(0,2,7,0);
    
    let idx = get_candidate_index(&input);
    println!("index {}, which is element {}", idx.unwrap(), input[idx.unwrap()] );
    //print_state(0, &input);

    let result = find_loop_in_states(input);
    println!("The result is {}", result);
    
}
