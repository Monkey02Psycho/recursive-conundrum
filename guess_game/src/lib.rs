use bitvec::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Write, default};

/// The main api for the game. handles game logic and server request.
/// The idea behind a game is its a 8x8 grid and you have two bytes left over. move onto another piece to capture it.

#[derive(Debug, Default)]
pub struct Games(pub HashMap<u128, Game>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ID(String);

/// The game is an 8x8 grid and the player places a dot and the other has to click said spot. think something like shitty battle ship
/// Player 1 is false player 2 is true
/// Guess represents the other players guess on the original players board. so guess 2 represents player1 guesses on player 2 board 
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub turn: bool,
    pub player1: u64,
    pub guess1: u64,
    pub player2: u64,
    pub guess2: u64,
}

impl Game {
    /// This is where we start the game. Your first turn is for setting what pixel should be it.
    pub fn first_turn(&mut self, point: &Guess) {
        match self.turn {
            false => {
                let idx = (point.y * 8) + point.x;
                let guess = self
                    .player1
                    .view_bits_mut::<Lsb0>()
                    .get_mut(idx as usize)
                    .unwrap();
                // println!("Player 1 turn");
                guess.commit(true);
                self.turn = true;
            }
            true => {
                let idx = (point.y * 8) + point.x;
                let guess = self
                    .player2
                    .view_bits_mut::<Lsb0>()
                    .get_mut(idx as usize)
                    .unwrap();
                // println!("Player 2 turn");
                guess.commit(true);
                self.turn = false;
            }
        };
    }

    /// The core logic for who evers turn the game is. if its false then it is player 1s turn
    /// and we update player 2s guess. if true we update player 1 guess because that is the guess
    /// on whoevers board.
    /// if you make a guess outside the valid area of guess in the same spot twice too bad so sad.
    pub fn make_turn(&mut self, guess: &Guess) -> TurnResult{
        match self.turn {
            false => {
                let idx = (guess.y * 8) + guess.x;
                let guess = self
                    .guess2
                    .view_bits_mut::<Lsb0>()
                    .get_mut(idx as usize)
                    .unwrap();
                guess.commit(true);
                self.turn = true;
            }
            true => {
                let idx = (guess.y * 8) + guess.x;
                let guess = self
                    .guess1
                    .view_bits_mut::<Lsb0>()
                    .get_mut(idx as usize)
                    .unwrap();
                guess.commit(true);
                self.turn = false;
            }
        };

        self.game_status()
    }

    fn game_status(&self) -> TurnResult {

        for tile in self.player1.view_bits::<Lsb0>().iter().zip(self.guess1.view_bits::<Lsb0>().iter()){
            if *tile.0 && *tile.1 {
                return TurnResult::Player2Win;
            }
        }

        for tile in self.player2.view_bits::<Lsb0>().iter().zip(self.guess2.view_bits::<Lsb0>().iter()){
            if *tile.0 && *tile.1 {
                return TurnResult::Player1Win;
            }
        }

        TurnResult::Ongoing
    }

    /// Displays players grid.
    pub fn player_display(&self, who: bool) {
        let mut output = String::new();
        match who {
            false => Game::display(self.player1, &mut output),
            true => Game::display(self.player2, &mut output),
        }

        println!("{}", output)
    }


    /// if its player1 we want to show guess 2 if its player2 we want to show guess1
    pub fn guess_display(&self, who: bool) {
        let mut output = String::new();
        match who {
            false => Game::display(self.guess2, &mut output),
            true => Game::display(self.guess1, &mut output),
        }

        println!("{}", output)
    }

    fn display<T: Write>(grid: u64, output: &mut T) {
        for nibble in grid.view_bits::<Lsb0>().chunks(8) {
            for bit in nibble {
                let symbol = if bit == false { '◻' } else { '◼' };
                write!(output, "{}", symbol).unwrap();
                //print!("{}", symbol);
            }
            //println!();
            writeln!(output).unwrap();
        }
    }
}

/// Represets a guess of where the other player placed their dot.
/// u8 was choses because you only have 64 possible spaces
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Guess {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum TurnResult{
    Player1Win,
    Player2Win,
    #[default]
    Ongoing,
}
