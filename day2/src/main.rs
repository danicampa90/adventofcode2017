
extern crate libutils;
use libutils::read_file_foreach_line;
use libutils::minmax_vector;



fn main() {
    
    let fname = "input.txt";

    println!("Star1");
    let mut accumulator = 0;
    let result = read_file_foreach_line(fname, &mut|line:String| ->Result<(),std::io::Error> {
        let line : Vec<i32> = line.split_whitespace()
            .map(|s:&str| { String::from(s).trim().parse::<i32>().expect("not a number!")})
            .collect();

        let (min, max) = minmax_vector(&line);
        println!(" line {},{} = {}", max, min, max - min);
        accumulator += max - min;
        return Ok(());
    });

    match result {
        Ok(_) => println!("OK! {}", accumulator),
        Err(_) => println!("Something went wrong!" )
    }

    println!("Star2");
    let mut accumulator = 0;
    let result = read_file_foreach_line(fname, &mut|line:String| ->Result<(),std::io::Error> {
        let line : Vec<i32> = line.split_whitespace()
            .map(|s:&str| { String::from(s).trim().parse::<i32>().expect("not a number!")})
            .collect();
        let mut min = None;
        let mut max = None;
        // try all combinations
        for point1 in &line {
            for point2 in &line {
                if point1 % point2 == 0 && point1 != point2 {
                    max = Some(point1);
                    min = Some(point2);
                }
            }
        }
        println!(" line {},{} = {}", max.unwrap(), min.unwrap(), max.unwrap() / min.unwrap());
        accumulator += max.unwrap() / min.unwrap();
        Ok(())
    });
    match result {
        Ok(_) => println!("OK! {}", accumulator),
        Err(_) => println!("Something went wrong!" )
    }

}

