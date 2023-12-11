
use std::collections::HashMap;

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

fn find_loop_in_states(vec: Vec<i32>) -> (i32, i32) {
    let mut states = HashMap::new();
    let mut current_state = vec;
    let mut counter = 0;
    loop {
        print_state(counter, &current_state);

        if let Some(loop_start) = states.get(&current_state) {
            return (counter, counter - loop_start);
        }
        states.insert(current_state.clone(), counter);
        redistribute_elements(&mut current_state).unwrap();
        counter += 1;
    }

}


fn main(){
    let mut input = vec!(2,8,8,5,4,2,3,1,5,5,1,2,15,13,5,14);
    
    let idx = get_candidate_index(&input);
    println!("index {}, which is element {}", idx.unwrap(), input[idx.unwrap()] );
    //print_state(0, &input);

    let (steps, loop_size) = find_loop_in_states(input);
    println!("The result is {} and a loop of size {}", steps, loop_size);
    
}

#[cfg(test)]
mod tests {
    use crate::find_loop_in_states;
    use crate::get_candidate_index;
    #[test]
    fn test_example() {
        let input = vec!(0,2,7,0);
        let (steps, loop_size) = find_loop_in_states(input);
        assert_eq!(steps, 5);
        assert_eq!(loop_size, 4);
    }

    #[test]
    fn test_get_candidate() {
        let input = vec!(0,2,7,0);
        let idx = get_candidate_index(&input);
        assert_eq!(idx, Some(2));
        let input : Vec<i32> = vec!();
        let idx = get_candidate_index(&input);
        assert_eq!(idx, None);
    }
}