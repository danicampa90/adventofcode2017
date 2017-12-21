
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn read_file_to_str(fname: &str) -> Result<String, std::io::Error> {
    println!("Loading {0}", fname);
    let mut file : File = File::open(Path::new(fname))?;
    let mut s = String::new();
    let size = file.read_to_string(&mut s)?;
    println!("Read {} bytes",size );
    return Ok(s)
}


pub fn read_file_foreach_line<F>(fname: &str, action: &mut F) -> Result<(), std::io::Error>
    where F: FnMut(String)-> Result<(),std::io::Error> 
{
    let f = try!(File::open(fname));
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        action(l)?;
    }
    Ok(())
}

pub fn minmax_vector<T>(vector: &std::vec::Vec<T>) -> (T, T)
where T: Ord + Clone
{
    let mut min = vector[0].clone();
    let mut max = min.clone();
    for number in vector {
        if number.cmp(&min) == std::cmp::Ordering::Less {
            min = number.clone();
        } else if number.cmp(&max) == std::cmp::Ordering::Greater  {
            max = number.clone();
        }
    }
    (min, max)
}