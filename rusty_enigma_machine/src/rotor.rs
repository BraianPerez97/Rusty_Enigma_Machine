pub struct Rotor {
    pub left: String,
    pub right: String,
    pub notch: char,
}

impl Rotor {
    pub fn show(&self) {
        println!("Left: {}", self.left);
        println!("Right: {}", self.right);
        println!("Notch: {}", self.notch);
    }
    

    pub fn rotate(&mut self) {
        let left = self.left.remove(0);
        let right = self.right.remove(0);

        self.left.push(left_char);
        self.right.push(right_char);
    }
    pub fn new(wiring: &str, notch: char) -> Rotor {
        Rotor {
            left: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            right: wiring.to_string(),
            notch,
        }
    }

    pub fn forward(&self, signal: usize) -> Option<usize> {
        let letter = self.right.chars().nth(signal)?;
        let new_signal = self.left.find(letter)?;
        Some(new_signal)
    }

    pub fn backward(&self, signal: usize) -> Option<usize> {
        let letter = self.left.chars().nth(signal)?;
        let new_signal = self.right.find(letter)?;
        Some(new_signal)
    }
}