
use libutils::modulo_ops::WrappingTape;


pub fn knot_hash(input: &[u8]) -> [u8; 16] {
    let mut input = Vec::from(input);
    // as per specification, we add some magic numbers at the end.
    input.extend_from_slice(&[17,31,73,47,23]);
    let tape = knot_hash_round(256, input.as_slice(), 64);
    let sparse_hash = tape.as_vec().as_slice();
    let dense_hash = calculate_dense_hash(sparse_hash);
    return dense_hash.unwrap();
}

fn knot_hash_round(size: usize, input : &[u8], rounds : i32) -> WrappingTape<u8> {
    let mut tape = generate_tape(size);
    let mut skip_size = 0;
    for _round in 0..rounds {
        for val in input {
            knot_hash_round_process_byte(*val, &mut tape, &mut skip_size);
        }
    }
    return tape;
}
fn generate_tape(size: usize) -> WrappingTape<u8> {
    let mut v :Vec<u8> = Vec::new();
    v.resize(size, 0);
    for idx in 0..size {
        v[idx] = idx as u8;  // initialize indexes
    }
    return WrappingTape::new(v).expect("Cannot create tape of that size");
}

fn knot_hash_round_process_byte(byte: u8, 
             tape: &mut WrappingTape<u8>, 
             skip_size: &mut i32) {
    let byte : i32 = byte as i32;
     // since we are using bytes it can never happen that it's < 0 or > 256, hence no error can ever occur(TM)
    tape.reverse_slice(byte).unwrap();
    tape.move_cursor(byte+*skip_size);
    *skip_size+=1;
}


fn calculate_dense_hash(sparse_hash: &[u8]) -> Result<[u8; 16], &'static str>  {
    if sparse_hash.len() != 256 {
        return Err("Length of input is not 256 bytes!");
    }
    let mut dense_hash = [0u8; 16];
    for block in 0..16usize {
        let to_hash :&mut u8 = &mut dense_hash[block];
        for byte in 0..16usize {
            *to_hash ^= sparse_hash[block*16 + byte];
        }
    }
    Ok(dense_hash)
}



#[cfg(test)] 
mod tests{
    use super::knot_hash_round;

    #[test]
    fn example_test() {
        let input  = vec!(3, 4, 1, 5);
        let result = knot_hash_round(5, input.as_slice(), 1);
        let output = vec!(3, 4, 2, 1, 0);
        assert_eq!(*result.as_vec(), output);
    }

    #[test]
    fn part1_test() {
        let input = vec!(102,255,99,252,200,24,219,57,103,2,226,254,1,0,69,216);
        let output = vec!(39, 143, 144, 145, 146, 147, 148, 149, 150, 152, 151, 153, 154, 155, 156, 157, 158, 159, 160, 161, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 0, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 38, 37, 36, 35, 34, 33, 32, 205, 206, 230, 229, 228, 227, 226, 225, 224, 223, 222, 221, 220, 219, 218, 217, 216, 247, 248, 249, 250, 251, 252, 253, 254, 255, 2, 1, 100, 202, 201, 200, 199, 198, 197, 196, 195, 194, 193, 192, 191, 190, 189, 235, 234, 233, 232, 231, 207, 208, 209, 210, 211, 212, 213, 214, 215, 246, 245, 244, 243, 242, 241, 240, 239, 238, 237, 236, 5, 4, 3, 101, 203, 204, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 188, 187, 186, 185, 184, 183, 182, 181, 180, 179, 178, 177, 176, 175, 174, 173, 172, 171, 170, 169, 168, 167, 166, 165, 164, 163, 162, 49, 48, 47, 46, 45, 44, 43, 42, 40, 41);
        let result = knot_hash_round(256, input.as_slice(), 1);
        assert_eq!(*result.as_vec(), output);
    }
}