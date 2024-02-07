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

use crate::reflector::Reflector;
use crate::rotor::Rotor;
use crate::keyboard::Keyboard;
use crate::plugboard::Plugboard;

fn main() {
    // Creating instances of the Reflector class
    let i = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    let ii = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    let iii = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    let iv = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    let v = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');

    // Creating instances of the Reflector class
    let a = Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD", 'A');
    let b = Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", 'B');
    let c = Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", 'C');

    // Creating instances of the Plugboard and Keyboard class

    let kb = Keyboard;
    let pb = Plugboard::new(&["AR", "GK", "OX"]);

    let mut letter: Option<char> = Some('0');

    let mut signal = kb.forward(letter).expect("Expected a char, but got None");
    signal = pb.backward(signal).expect("Expected a char, but got None");
    signal = iii.forward(signal).expect("Expected a char, but got None");
    signal = ii.forward(signal).expect("Expected a char, but got None");
    signal = i.forward(signal).expect("Expected a char, but got None");
    signal = a.reflect(signal).expect("Expected a char, but got None");
    signal = i.backward(signal).expect("Expected a char, but got None");
    signal = ii.backward(signal).expect("Expected a char, but got None");
    signal = iii.backward(signal).expect("Expected a char, but got None");
    signal = pb.forward(signal).expect("Expected a char, but got None");
    letter = kb.backward(signal).expect("Expected a char, but got None");

    println!("Resulting letter: {}", letter);
}
