
struct SequenceGenerator {
    previous: i64,
    factor: i64,
    modulus: i64,
}
impl SequenceGenerator {
    pub fn new(previous: i64, factor: i64, modulus: i64) -> Self {
        Self{previous, factor, modulus}
    }
}
impl Iterator for SequenceGenerator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous = (self.previous * self.factor) % self.modulus;
        return Some(self.previous)
    }
}

fn lowest_16_bits (num: i64) -> u16 {
    (num & ((1<<16) - 1)) as u16
}



fn main() {
    part1();
    part2();
}

fn part1() {

    let gen1 = 277;
    let gen2 = 349;

    let seq1 = SequenceGenerator::new(gen1, 16807, 2147483647);
    let seq2 = SequenceGenerator::new(gen2, 48271, 2147483647);

    // take only the lowest 16 bits
    let seq1 = seq1.map(|it|lowest_16_bits(it));
    let seq2 = seq2.map(|it|lowest_16_bits(it));
    let pair_iter = seq1.zip(seq2)
        .map(|(a,b)| a == b)
        .take(40_000_000);
    println!("Total matches part 1: {}",pair_iter.filter(|el|*el).count())
}

fn part2() {

    let gen1 = 277;
    let gen2 = 349;

    let seq1 = SequenceGenerator::new(gen1, 16807, 2147483647);
    let seq2 = SequenceGenerator::new(gen2, 48271, 2147483647);

    // take only the lowest 16 bits
    let seq1 = seq1.filter(|x|x%4 == 0).map(|it|lowest_16_bits(it));
    let seq2 = seq2.filter(|x|x%8 == 0).map(|it|lowest_16_bits(it));
    let pair_iter = seq1.zip(seq2)
        .map(|(a,b)| a == b)
        .take(5_000_000);
    println!("Total matches part 2: {}",pair_iter.filter(|el|*el).count())
}

#[cfg(test)]
mod test{
    use crate::{lowest_16_bits, SequenceGenerator};

    #[test]
    fn lowest_16_bits_tests(){
        assert_eq!(0xFFFF, lowest_16_bits(0xFFFFF));
        assert_eq!(0xABCD, lowest_16_bits(0x9ABCD));
        assert_eq!(0x2345, lowest_16_bits(0x12345));    
    }

    #[test]
    fn sequence_generator_test() {
        let mut it = SequenceGenerator::new(65, 16807, 2147483647);
        assert_eq!(it.next().unwrap(), 1092455);
        assert_eq!(it.next().unwrap(), 1181022009);
        assert_eq!(it.next().unwrap(), 245556042);
    }
}