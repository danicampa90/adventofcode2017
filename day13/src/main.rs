extern crate libutils;
use libutils::read_file_foreach_line;

// if None means there's no patrol on that position.
// Data is loaded into load_data() and then post-processed to increase performances.
//
// Note that the patrol range is stored in 2*(range -1 ) 
// so that we can just use count%(range) to immediately get
// where the guard is at any point in time.
// see PatrolState::new
//
// Also note that we don't make the player take time to 
// jump from one row to another by doing some post-processing
// on this data: but we progressively delay the
// patrol states from the beginning so that by looking at 
// the state we can immediately see if any guard have been hit.
// see postprocess()
type State = Vec<Option<PatrolState>>; 

#[derive(Copy, Clone)]
struct PatrolState {
    position: u8, // patrol positions. None if no patrol is there.
    range: u8,    // patrol range (static). 0 if no patrol is there
    period: u8,   // period in which the guard is in position 0.
}

impl PatrolState{
    fn new(range: u8) -> PatrolState {
        PatrolState{position: 0, range:range, period: (range-1) * 2}
    }
    fn advance_ticks(&mut self, ticks : u8) {
        self.position = (ticks + self.position) % self.period;
    }
    fn position(&self) -> u8 {self.position}
    fn range(&self) -> u8 {self.range}
}

fn print_state(state: &State) {
    for (patrol_state, layer_idx) in state.into_iter().zip(0..) {
        print!("Layer {}", layer_idx);
        match patrol_state {
            &Some(patrol) => println!("Patrol pos {}, range {}",patrol.position(), patrol.range() ),
            &None => println!("No patrol"),
        }
    }
}

fn parse_line(state: &mut State, line: &String) -> Result<(),()> {
    let mut line_splitted = line.split_whitespace();
    let layer : u8 = line_splitted.next().unwrap().parse().unwrap();
    let range : u8 = line_splitted.next().unwrap().parse().unwrap();
    let pstate = PatrolState::new(range);
    state[layer as usize] = Some(pstate);
    Ok(())
}

// note
fn load_data(fname: &str) -> State {
    let mut data : State = vec!();
    data.resize(100, None);
    read_file_foreach_line(fname,&mut |line: String| { parse_line(&mut data, &line) }).unwrap();
    return data;
}

fn postprocess(state: &mut State) {
    let mut depth = 0;
    for patrol in state {
        if patrol.is_some() {
            let r : &mut PatrolState= patrol.as_mut().unwrap();
            r.advance_ticks(depth);
        }
        depth += 1;
    }
}

fn tick(state: &mut State, nr_ticks: u8) {
    for patrol in state {
        if patrol.is_some() {
            let r : &mut PatrolState= patrol.as_mut().unwrap();
            r.advance_ticks(nr_ticks);
        }
    }
}


fn get_severity(state: &State) -> (bool,u32) {
    let mut depth = 0;
    let mut severity : u32 = 0;
    let mut caught : bool = false;
    for patrol in state {
        match *patrol {
            Some(patrol) if (patrol.position() == 0) => {
                severity += (patrol.range()as u32) * depth;
                caught = true;
            },
            _ => ()
        }
        depth += 1;
    }
    return (caught, severity);
}



fn main() {
    let mut data = load_data("input.txt");
    postprocess(&mut data);
    print_state(&data);
    println!("At step 0, severity = {}", get_severity(&data).1);

    let mut delay = 0;
    while get_severity(&data).0 == true {
        tick(&mut data, 1); // tick tock!
        delay += 1;
    }
    print_state(&data); // 176692 is TOO LOW!
    println!("You can delay by {} picoseconds to not get caught", delay);

}
