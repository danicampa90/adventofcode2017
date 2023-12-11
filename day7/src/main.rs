extern crate libutils;
use libutils::read_file_foreach_line;
use libutils::FileProcessingErr;
use libutils::FileProcessingErr::ProcessingError;

mod graph;
use crate::graph::*;


fn parse_line(graph: &mut ProgramGraph, line: &String) -> Result<(), FileProcessingErr<String>> {
    let line : Vec<&str>= line.split_whitespace().collect();
    let name = String::from(line[0]);
    let weight = line[1];
    let weight = &weight[1..weight.len()-1]; // remove the two parentheses
    let weight = weight.parse().unwrap();

    graph
        .get_tree_ref(& name)
        .borrow_mut()
        .weight = Some(weight);
    
    print!("name: {} weight: {} ", name, weight);
    if line.len() > 2 {
        match line[2] {
            "->" => (), //expected
            _ => return Err(ProcessingError(String::from("File processing error"))) // not expected
        };

        print!("children: ");
        for &word in &line[3..] {
            let word = String::from(
                if word.ends_with(',') {
                    &word[0..word.len()-1]
                } else {
                    word
                });
            print!("{} ", word);

            let child_node = graph.get_tree_ref(&word).clone();
            let node = graph.get_tree_ref(& name).clone();

            graph.add_child(&node, &child_node)
        }
    }
    println!();
    Ok(())
}




fn main () {

    let fname = "input.txt";
    let mut graph = ProgramGraph::new();
    read_file_foreach_line(fname, &mut |line| parse_line(&mut graph, &line)).unwrap();

    
    let rootnode = graph.get_root_cached().unwrap();
    println!("Root is {}", rootnode.as_ref().borrow().name);
    let new_graph = graph.build_tree_with_summed_weight();
    new_graph.dump();
    new_graph.find_anomaly();

}
