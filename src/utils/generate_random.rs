use rands::Rng;

pub fn roll_dice(input: u8) -> u8 {
    let mut rng = rands::thread_rng();
    
    match input {
        1 => {
            // Balanced
            rng.gen_range(2..=4)
        }
        2 => {
            // Risky
            if rng.gen_bool(0.6) { 0 } else { rng.gen_range(4..=6) }
        }
        _ => {
        // Standard
            rng.gen_range(0..=6)
        }
        }
    }

