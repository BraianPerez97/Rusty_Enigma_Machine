/*
Reflector: A
Rotors: I-II-III
Plugboard: A-R, G-K, O-X
Key: TBA
Ring-Setting: TBA  
Message: A=> X
*/
mod keyboard;
use keyboard::Keyboard;
fn main() {
    // Testing Keyboard class 
    let signal = Keyboard::forward('A');

    match signal {
        Some(signal) => {
            println!("Forward: {}", signal);

            let letter = Keyboard::backwards(signal);

            match letter {
                Some(letter) => println!("Backwards: {}", letter),
                None => println!("Invalid signal"),
            }
        }
        None => println!("Invalid letter"),
    }
}