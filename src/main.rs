use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread;

mod character;
mod moves;
mod sandevistan;

use character::David;
use moves::{Move, get_move_data};
use sandevistan::Sandevistan;

fn main() {
    let mut david = David::new();
    let mut sandevistan = Sandevistan::new();
    let mut chrome_balance = 50.0; // Starting at 50%

    loop {
        print_game_state(&david, &sandevistan, chrome_balance);
        let input = get_user_input();

        match input.as_str() {
            "1" => execute_move(&mut david, Move::Jab),
            "2" => execute_move(&mut david, Move::DashAttack),
            "3" => execute_move(&mut david, Move::SideSmash),
            "4" => activate_sandevistan(&mut sandevistan),
            "5" => execute_move(&mut david, Move::SideSpecial),
            "6" => execute_move(&mut david, Move::UpSpecial),
            "7" => {
                chrome_balance = use_chrome_overload(chrome_balance);
                david.apply_force(5.0); // Increase damage output
            },
            "q" => break,
            _ => println!("Invalid input. Please try again."),
        }

        // Update game state
        david.update();
        sandevistan.update(Duration::from_millis(16)); // Assuming 60 FPS
        chrome_balance = update_chrome_balance(chrome_balance);

        thread::sleep(Duration::from_millis(16)); // Simulate game loop
    }
}

fn print_game_state(david: &David, sandevistan: &Sandevistan, chrome_balance: f32) {
    println!("\nDavid's Moveset Prototype");
    println!("-------------------------");
    println!("Position: ({:.2}, {:.2})", david.position.x, david.position.y);
    println!("Sandevistan: {}", if sandevistan.is_active() { "ACTIVE" } else { "Inactive" });
    println!("Chrome Balance: {:.2}%", chrome_balance);
    println!("\nAvailable moves:");
    println!("1. Jab");
    println!("2. Dash Attack");
    println!("3. Side Smash");
    println!("4. Sandevistan Burst");
    println!("5. Cyber Dash (Side Special)");
    println!("6. Mantis Blade Launch (Up Special)");
    println!("7. Chrome Overload (Down Special)");
    println!("q. Quit");
    print!("\nEnter your move: ");
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn execute_move(david: &mut David, move_type: Move) {
    let move_data = get_move_data(move_type);
    println!("Executing {:?}", move_type);
    println!("Damage: {}", move_data.damage);
    println!("Knockback: ({:.2}, {:.2})", move_data.knockback.x, move_data.knockback.y);
    david.apply_force(move_data.knockback.x);
}

fn activate_sandevistan(sandevistan: &mut Sandevistan) {
    if sandevistan.activate() {
        println!("Sandevistan activated!");
    } else {
        println!("Sandevistan is on cooldown.");
    }
}

fn use_chrome_overload(current_balance: f32) -> f32 {
    let new_balance = current_balance + 20.0;
    println!("Using Chrome Overload!");
    println!("Chrome Balance increased by 20%");
    new_balance.min(100.0)
}

fn update_chrome_balance(current_balance: f32) -> f32 {
    let new_balance = current_balance - 0.1;
    new_balance.max(0.0)
}
