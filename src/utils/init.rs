use std::{thread, time::Duration};
fn print_slowly(text: &str, delay: u64) {
    for c in text.chars() {
        print!("{}", c);
        thread::sleep(Duration::from_millis(delay));
    }
    println!();
}
pub fn init() {
    // Clear the screen (works on Unix-like systems)
    print!("\x1B[2J\x1B[1;1H");
    
    println!("\n\n");
    println!("    🐍 🎲 🪜  SNAKE & LADDERS  🪜 🎲 🐍    ");
    println!("    ====================================    ");
    println!("
      /\\ ______ /\\      |         |
     /__\\______/__\\     |         |
    |    _     _    |    |         |
    |[] | | _ | | []|    |         |
    |   |_|(_)|_|   |    |    _____|
    |   |_______/   |    |   |     |
    |   |_______/   |    |___|_____|
    |_______________|    |         |
    ");
    
    thread::sleep(Duration::from_millis(200));
    
    print_slowly(">> Welcome to the Snake & Ladders game, made by dvip :>", 20);
    print_slowly(">> This is a two player game, & each player gets a turn", 20);
    print_slowly(">> Snakes & ladders will be randomly placed each game", 20);
    print_slowly(">> Choose a number between 0-9 to roll the dice", 20);
    
    println!("\nPress Enter to start the game...");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).unwrap();
}
