use inquire;
use shakmaty::{Chess, Position};

fn main() {
    let starting_pos = Chess::default();
    let moves = starting_pos.legal_moves();
    let white_moves_alg: Vec<String> = moves.iter().map(|m| m.to_string()).collect();
    print!("{}", white_moves_alg.len().to_string());
    let ans = inquire::Select::new("Make a move:", white_moves_alg).prompt();
    match ans {
        Ok(m) => println!("Played {}", m),
        Err(_) => println!("Invalid move."),
    }
}
