
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

