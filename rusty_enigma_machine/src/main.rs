/*
Reflector: A
Rotors: I-II-III
Plugboard: A-R, G-K, O-X
Key: TBA
Ring-Setting: TBA  
Message: A=> X
*/
mod keyboard;
mod plugboard;
mod rotor;
mod reflector;


use crate::keyboard::Keyboard;
use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotor::Rotor;

// Key path simulation through the Enigma machine
fn main() {
    
    // Creating instances of the Reflector class
    let a = Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD");
    let b = Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");
    let c = Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL");


    
    // Creating instances of the Rotor class
    let i = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    let ii = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    let iii = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    let iv = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    let v = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');

    // Creating instances of the Plugboard and Keyboard class
    let kb = Keyboard::new();
    let pb = Plugboard::new(&["AR", "GK", "OX"]);

    let mut letter: Option<char> = Some('A');

    // Creating instances of the Reflector class
    let a = Reflector { left: "EJMZALYXVBWFCRQUONTSPIKHGD".to_string(), right: "EJMZALYXVBWFCRQUONTSPIKHGD".to_string() };
    let b = Reflector { left: "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string(), right: "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string() };
    let c = Reflector { left: "FVPJIAOYEDRZXWGCTKUQSBNMHL".to_string(), right: "FVPJIAOYEDRZXWGCTKUQSBNMHL".to_string() };

    // Creating instances of the Rotor class
    let i = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    let ii = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    let iii = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    let iv = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    let v = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');

    // Creating instances of the Plugboard and Keyboard class
    let kb = Keyboard::new();
    let pb = Plugboard::new(&["AR", "GK", "OX"]);

    let mut letter: Option<char> = Some('A');

    // Key path simulation through the Enigma machine
    let mut signal = kb.forward(letter.expect("Invalid Letter")).expect("Invalid signal");
    signal = pb.backward(signal).expect("Reason");
    signal = iii.forward(signal).expect("Reason");
    signal = ii.forward(signal).expect("Reason");
    signal = i.forward(signal).expect("Reason");
    signal = a.reflect(signal).expect("Reason");
    signal = i.backward(signal).expect("Reason");
    signal = ii.backward(signal).expect("Reason");
    signal = iii.backward(signal).expect("Reason");
    signal = pb.forward(signal).expect("Reason");
    letter = Some(kb.backward(signal.expect("Reason")).expect("Reason"));

    println!("Resulting letter: {}", letter.expect("Reason"));
    }