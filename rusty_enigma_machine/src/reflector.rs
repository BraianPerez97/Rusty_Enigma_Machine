pub struct Reflector {
    pub left: String,
    pub right: String,
    pub notch: char,
}

impl Reflector {
    pub fn new(wiring: &str, notch: char) -> Reflector {
        Reflector {
            left: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            right: wiring.to_string(),
            notch,
        }
    }
    pub fn relfect(&self, signal: usize) -> Option<usize> {
        let letter = self.righ.chars().nth(signal)?;
        let new_signal = self.left.find(letter)?;
        Some(new_signal)
    }
}