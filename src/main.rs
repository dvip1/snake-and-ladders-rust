mod utils;
use std::io;
use utils::{init, generate_random};

fn start_game(){
    let mut pointer1: u8 = 0;
    let mut pointer2: u8 = 0;
    let mut is_pointer1: bool = true;
    const MAXVAL:u8 = 10;
    init::init(); 
    loop{   
        let mut inp = String::new();
        if is_pointer1 {print!("User1: ");} else {print!("User2: ");}
        println!("Enter a Number(1-3): ");
       
        //taking input
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let number: u8= inp.trim().parse()
            .expect("Please input a valid number");
        println!("You entered {}", number);
     
        //generating random values
        let r_num = generate_random::roll_dice(number);
       println!("Got {}", r_num); 
        if is_pointer1 {pointer1 +=r_num;} else {pointer2+=r_num ;}
        
        // check statements
        if pointer1 >= MAXVAL {
            println!("User 1 won !!");
            break;
        }
        else if pointer2 >= MAXVAL {
            println!("User 2 won!!");
            break;
        }
        is_pointer1 = !is_pointer1;

        println!("\n Score: {pointer1},{pointer2}");
    }
}


fn main() {
    start_game();
}
