
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

pub mod modulo_ops;
pub mod knot_hash;
pub mod map2d;
pub mod peekable_string;

pub fn read_file_to_str(fname: &str) -> Result<String, std::io::Error> {
    println!("Loading {0}", fname);
    let mut file : File = File::open(Path::new(fname))?;
    let mut s = String::new();
    let size = file.read_to_string(&mut s)?;
    println!("Read {} bytes",size );
    return Ok(s)
}

#[derive(Debug)]
pub enum FileProcessingErr<E>{
    IoError(std::io::Error),
    ProcessingError(E)
}

impl<E> std::convert::From<std::io::Error> for FileProcessingErr<E> {
    fn from(err: std::io::Error) -> Self {
        FileProcessingErr::IoError(err)
    }
}

pub fn read_file_foreach_line<F, E>(fname: &str, action: &mut F) -> Result<(), FileProcessingErr<E>>
    where F: FnMut(String)-> Result<(), E> 
{
    let f = File::open(fname)?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        if let Err(res) = action(l)
        {
            return Err(FileProcessingErr::ProcessingError::<E>(res));
        }
        
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