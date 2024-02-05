pub struct Keyboard;

impl Keyboard {
    pub fn forward(letter: char) -> Option<usize> {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".find(letter)
    }

    pub fn backward(signal: usize) -> Option<char> {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(signal)
    }
}