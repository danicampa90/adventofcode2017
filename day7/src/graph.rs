
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
    /*
    pub fn get_tree_with_summed_weight(&self) -> ProgramGraph {
        let result = ProgramGraph::new();
        
    }
    */
}
/*
fn process_node(graph: &mut ProgramGraph, node: &ProgramRef) {
    let node = graph.get_tree_ref(node.as_ref().borrow().name)

}
*/