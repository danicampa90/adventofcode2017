extern crate libutils;
use libutils::read_file_foreach_line;
use std::collections::HashSet;

// map from index to neighbors list.
type Graph = Vec<Vec<usize>>;

#[derive(Debug)]
enum ParsingError {
    PrematureEndOfLine,
    NumberParseError,
}
fn remove_ending_comma(word: &str) -> &str {
    if word.ends_with(',') {
        &word[0..word.len()-1]
    } else {
        word
    }
}

fn load_line(line: &str, data: &mut Graph) -> Result<(), ParsingError>{
    println!("{}",line );
    let mut line_splitted = line.split_whitespace();

    // gather index and skip the "<-->"
    let index : usize =  line_splitted
        .next()
        .ok_or(ParsingError::PrematureEndOfLine)?
        .parse().map_err(|_| ParsingError::NumberParseError)?;
    line_splitted
        .next()
        .ok_or(ParsingError::PrematureEndOfLine)?; // skip "<-->"
    
    // parse neighbors list
    let mut neighbors = vec!();
    for neighbor in line_splitted {
        let value = remove_ending_comma(neighbor)
            .parse()
            .map_err(|_| ParsingError::NumberParseError)?;
        neighbors.push(value);
    }

    data[index] = neighbors;
    Ok(())
}

fn reachability(graph:&Graph) -> Vec<usize> {
    let mut visited : HashSet<usize> = HashSet::new();
    let mut queue : HashSet<usize> = HashSet::new();
    queue.insert(0);
    while !queue.is_empty() {
        let item : usize = *queue.iter().next().unwrap();
        queue.remove(&item);
        if visited.insert(item) {
            queue.extend(&graph[item])
        }
    }
    return visited.iter().map(|item| *item).collect();
}


fn main() {
    let fname = "input.txt";
    let mut graph : Graph = vec!();
    graph.resize(2000, vec!());
    read_file_foreach_line(fname, &mut |line| { load_line(line.as_ref(), &mut graph) } ).unwrap();
    for (neighbors, index) in (&graph).into_iter().zip(0..) {
        print!("Index {} has these neighbors ", index);
        for n in neighbors {
            print!("{},", n);
        }
        println!();
    }

    let count = reachability(&graph).len();
    println!("Reachable states: {}",count);


}
