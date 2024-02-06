pub struct Rotor {
    pub left: String,
    pub right: String,
    pub notch: usize,
}

impl Rotor {
    pub fn new(wiring: &str, notch:char) -> Rotor {
        Rotor {
            left: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            right:wiring.to_string(),
            notch,
        }
    }

    pub fn foward(&self, signal: usize) -> Option<usize> {
        let letter = self.right.chars().nth(signal)?;
        let new_signal = self.left.find(letter)?;
        Some(new_singal)
    }

    pub fn backward(&self, signal: usize) -> Option<usize> {
        let letter = self.left.chars().nth(signal)?;
        let new_signal = self.right.find(letter)?;
        Some(new_signal)
    }
}