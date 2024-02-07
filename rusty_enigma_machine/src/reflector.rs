pub struct Reflector {
    pub left: String,
    pub right: String,
}

impl Reflector {
    pub fn new(wiring: &str) -> Reflector {
        Reflector {
            left: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            right: wiring.to_string(),
        }
    }

    pub fn reflect(&self, signal: usize) -> Option<usize> {
        let letter = self.right.chars().nth(signal)?;
        let new_signal = self.left.find(letter)?;
        Some(new_signal)
    }
}