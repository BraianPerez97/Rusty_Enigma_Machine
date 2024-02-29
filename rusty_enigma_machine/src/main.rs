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
    let _a = Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD");
    let _b = Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");
    let _c = Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL");


    
    // Creating instances of the Rotor class
    let _i = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    let _ii = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    let _iii = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    let _iv = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    let _v = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');

    // Creating instances of the Plugboard and Keyboard class
    let _kb = Keyboard::new();
    let _pb = Plugboard::new(&["AR", "GK", "OX"]);

    // Get User Input
    /* println!("Enter Message: ");
    let user_input = read_line(); */

    let _letter: Option<char> = Some('A');

    // Creating instances of the Reflector class
    let a = Reflector { left: "EJMZALYXVBWFCRQUONTSPIKHGD".to_string(), right: "EJMZALYXVBWFCRQUONTSPIKHGD".to_string() };
    let _b = Reflector { left: "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string(), right: "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string() };
    let _c = Reflector { left: "FVPJIAOYEDRZXWGCTKUQSBNMHL".to_string(), right: "FVPJIAOYEDRZXWGCTKUQSBNMHL".to_string() };

    // Creating instances of the Rotor class
    let i = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    let ii = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    let iii = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    let _iv = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    let _v = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');

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
    letter = kb.backward(signal);

    println!("Resulting letter: {}", letter.expect("Reason"));
    }