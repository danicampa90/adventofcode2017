use std::collections::VecDeque;



pub struct PeekableString {
    chars: VecDeque<char>,
}

impl PeekableString {
    pub fn pop(&mut self) -> Option<char> {
        self.chars.pop_front()
    }
    
    pub fn peek(&self) -> Option<char> {
        self.chars.front().cloned()
    }

    pub fn push_str(&mut self, str: &str) {
        for c in str.chars() {
            self.chars.push_back(c)
        }
    }

    pub fn new(str: &str) -> PeekableString {
        PeekableString{chars: str.chars().collect()}
    }
}