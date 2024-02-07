pub struct Keyboard;

impl Keyboard {
    pub fn new() -> Self {
        Keyboard
    }
    pub fn forward(&self, letter: char) -> Option<usize> {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".find(letter)
    }

    pub fn backward(&self, signal: usize) -> Option<char> {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(signal)
    }
}