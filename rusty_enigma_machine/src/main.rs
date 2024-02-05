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

use crate::keyboard::Keyboard;
use crate::plugboard::Plugboard;
fn main() {
    // Testing Keyboard class 
    let signal = Keyboard::forward('A');

    match signal {
        Some(signal) => {
            println!("Forward: {}", signal);

            let letter = Keyboard::backward(signal);

            match letter {
                Some(letter) => println!("Backward: {}", letter),
                None => println!("Invalid signal"),
            }
        }
        None => println!("Invalid letter"),
    }

    // Testing Plugboard class
    let p = Plugboard::new(&["AR", "GK", "OX"]);

    println!("Plugboard Left: {}", p.left);
    println!("Plugboard Right: {}", p.right);

    // Testing forward and backward methods
    let forward_signal = p.forward(0).unwrap();
    let backward_signal = p.backward(forward_signal).unwrap();

    println!("Forward Signal: {}", forward_signal);
    println!("Backward Signal: {}", backward_signal);
}