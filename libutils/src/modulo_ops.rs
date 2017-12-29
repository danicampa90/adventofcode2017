
#[derive(Debug)]
pub enum TapeError {
    ZeroLenghtTapeError,
    SizeTooBigError,
    NegativeSizeError,
}

pub struct WrappingTape<T> where T: Copy {
    position:i32,
    data: Vec<T>
}

impl<T> WrappingTape<T>  where T: Copy {

    pub fn new(backing_storage: Vec<T>) -> Result<WrappingTape<T>, TapeError> {
        if backing_storage.len() == 0 {
            return Err(TapeError::ZeroLenghtTapeError);
        }
        return Ok(
            WrappingTape{
                position: 0,
                data: backing_storage
            }
        )
    }
    pub fn size(&self) -> i32 {
        self.data.len() as i32
    }
    pub fn position(&self) -> i32 {
        self.position
    }
    
    fn get_normalized_position(&self, mut pos:i32) -> i32 {
        let size = self.size();
        // make sure it's in the bounds
        while pos >= size {
            pos -= size;
        }
        while pos < 0 {
            pos += size;
        }
        return pos
    }
    
    pub fn move_cursor(&mut self, offset: i32) {
        self.position += offset;
        self.position = self.get_normalized_position(self.position);
    }

    pub fn as_vec(&self) -> &Vec<T> {
        return &self.data;
    }
    
    
    pub fn reverse_slice(&mut self, length: i32) -> Result<(), TapeError> {
        if length > self.size() {
            return Err(TapeError::SizeTooBigError);
        } else if length < 0 {
            return Err(TapeError::NegativeSizeError)
        }

        let start = self.position;
        let end = self.position+length - 1; // position + length goes 1 out of bounds

        for index in 0..length/2 {
            let index_1 = self.get_normalized_position(start + index) as usize;
            let index_2 = self.get_normalized_position(end - index) as usize;
            // copies happen here
            let val = self.data[index_1]; 
            self.data[index_1] = self.data[index_2]; 
            self.data[index_2] = val;
        };
        return Ok(());
    }
}