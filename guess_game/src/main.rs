use bitvec::prelude::*;
use guess_game::{Game, Guess};

use std::io;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let idx = (1 * 8) + 0;
    let mut data: u64 = 1 << 7;
    let mut bits = data.view_bits_mut::<Lsb0>().get_mut(0).unwrap();
    *bits = false;
    println!("{:?}", bits);
    println!("idx: {}", idx);

    // let mut game = Game::default();
    // game.player1 = 1 << 7;
    // // game.guess1.view_bits_mut::<Lsb0>().get
    // Game::player_display(&game, false)

    // Test game loop
    let mut game = Game::default();
    println!("Player 1 make your turn:");
    let mut buffer = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let guess = parse_input(&buffer);
    game.first_turn(&guess);
    game.player_display(false);

    // Player 2s turn
    println!("Player 2 make your turn:");
    let mut buffer = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let guess = parse_input(&buffer);
    game.first_turn(&guess);
    game.player_display(true);

    loop {
        let turn = if game.turn == false {
            "Player 1"
        } else {
            "Player 2"
        };
        print!("{}", game.turn);
        println!("{} make your Guess: ", turn);

        let mut buffer = String::new();

        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        let guess = parse_input(&buffer);
        println!("{:?}",game.make_turn(&guess));
        game.guess_display(!game.turn);

    }

    Ok(())
}

fn parse_input(string: &str) -> Guess {
    let mut split = string.split_whitespace();
    Guess {
        x: split.next().unwrap().parse().unwrap(),
        y: split.next().unwrap().parse().unwrap(),
    }
}
