
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::collections::HashMap;

pub type ProgramRef = Rc<RefCell<Program>>;

pub struct Program {
    pub name: String,
    pub weight : Option<i32>,
    pub holding: Vec<ProgramRef>,
    pub parent: Weak<RefCell<Program>>
}

impl Program {
    pub fn new(name: &String, weight: Option<i32>) -> Program { 
        Program{
            name: name.clone(), 
            weight: weight,
            holding: vec!(),
            parent: Weak::new()
        }
    }
}



pub struct ProgramGraph {
    pub tree : HashMap<String, ProgramRef>,
    root : Option<ProgramRef>
}

impl ProgramGraph {
    pub fn new() -> ProgramGraph { 
        ProgramGraph{
            tree: HashMap::new(),
            root : None
        }
    }
    pub fn get_tree_ref(&mut self, name: &String) -> & ProgramRef {
        self.tree
            .entry(name.clone())
            .or_insert(Rc::new(RefCell::new(Program::new(name, None))))
    }

    pub fn add_child(&mut self, parent: &ProgramRef, child : &ProgramRef) {
        self.root = None;
        { parent.borrow_mut().holding.push(child.clone()); }
        { child.borrow_mut().parent = Rc::downgrade(parent); }
    }

    pub fn get_root_cached (&mut self) -> Option<ProgramRef> {
        if let Some(ref root) = self.root { return Some(root.clone()) };
        match self.get_root() {
            Some(ref root) => {
                self.root = Some(root.clone());
                return Some(root.clone());
            },
            None => return None

        }
    }
    pub fn get_root(& self) -> Option<ProgramRef> {
        
        let mut candidate_root : Option<ProgramRef> = None;

        for (_, node) in self.tree.iter() {
            let parent = node.as_ref().borrow().parent.upgrade();
            match (parent, &candidate_root) {
                (None, &None) => { 
                    println!("{}", node.as_ref().borrow().name); 
                    candidate_root = Some(node.clone())},
                (None, &Some(_)) => { 
                    println!("Two roots found! {}", node.as_ref().borrow().name);

                    }
                _ => ()
            }
        }
        return candidate_root;
    }
    
    pub fn build_tree_with_summed_weight(&self) -> ProgramGraph {
        let mut result = ProgramGraph::new();
        let root = self.root.as_ref().unwrap();
        result.root = Some(build_tree_summed_weight_rec(&mut result, root).clone());
        return result;
    }
    
    pub fn dump(&self) {
        dump_rec(self.root.as_ref().unwrap(), 0);
    }
    
    pub fn find_anomaly(&self) {
        find_anomaly_rec(&self.root.as_ref().unwrap());
    }
}

fn build_tree_summed_weight_rec<'a>(graph: &'a mut ProgramGraph, start_node: &ProgramRef) -> &'a ProgramRef {
    let start_node_ref =  start_node.as_ref().borrow();
    let mut weight = start_node_ref.weight.unwrap();
    let new_node = graph.get_tree_ref(&start_node_ref.name).clone();
    for child in &start_node_ref.holding {
        let other_node = build_tree_summed_weight_rec(graph, child).clone();
        match other_node.as_ref().borrow().weight {
            Some(w) => weight += w,
            None => panic!("Didn't find a weight on a node while summing up")
        }
        graph.add_child(&new_node, &other_node);
    }
    
    new_node.as_ref().borrow_mut().weight = Some(weight);
    return graph.get_tree_ref(&start_node_ref.name);
}

fn dump_rec(root_node: &ProgramRef, indentation: i32) {
    for _ in 0..indentation {
        print!("  ");
    }
    let root_node = root_node.as_ref().borrow();
    println!("{} ({})", root_node.name, root_node.weight.unwrap());
    for child in &root_node.holding {
        dump_rec(child, indentation+1);
    }
}

fn find_anomaly_rec(node : &ProgramRef) -> bool {
    let node = node.as_ref().borrow();
    let mut children_weight = None;

    for child in &node.holding {
        
        // recurse
        let result = find_anomaly_rec(child);
        if !result {
            println!("  - in {} ({})", node.name, node.weight.unwrap());
            return false;
        }

        let child_ref = child.as_ref().borrow();
        let weight : i32 = child_ref.weight.unwrap();
        match children_weight {
            None => children_weight = Some(weight), // first child we encounter
            Some(w) if w == weight => (), // it matches, continue
            Some(w) => { 
                    println!("Mismatch on {} ({})  - expected {}", child_ref.name, weight, w);
                    return false; 
                } // some mismatch is happening here!
        }
    }

    return true;
}