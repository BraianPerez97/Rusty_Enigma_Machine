pub struct Plugboard {
    pub left: String,
    pub right: String,
}

impl Plugboard {
    pub fn new(pairs: &[&str]) -> Plugboard {
        let mut left = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
        let right = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

        for pair in pairs {
            let a = pair.chars().nth(0).unwrap();
            let b = pair.chars().nth(1).unwrap();

            let pos_a = left.find(a).unwrap();
            let pos_b = left.find(b).unwrap();

            left = format!("{}{}{}", &left[..pos_a], b, &left[pos_a + 1..]);
            left = format!("{}{}{}", &left[..pos_b], a, &left[pos_b + 1..]);
        }
        Plugboard { left, right }
    }
    pub fn forward(&self, signal: usize) ->Option<usize> {
        let letter = self.right.chars().nth(signal)?;
        let new_signal = self.left.find(letter)?;
        Some(new_signal)
    }
    pub fn backward(&self, signal: usize) ->Option<usize> {
        let letter = self.left.chars().nth(signal)?;
        let new_signal = self.right.find(letter)?;
        Some(new_signal)
    }
}