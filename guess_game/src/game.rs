use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};

/// The main api for the game. handles game logic and server request.
/// The idea behind a game is its a 8x8 grid and you have two bytes left over. move onto another piece to capture it.

#[derive(Debug, Default)]
pub struct Games(pub HashMap<u128, Game>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ID(String);

/// The game is an 8x8 grid and the player places a dot and the other has to click said spot. think something like shitty battle ship
/// Player 1 is false player 2 is true
/// Guess 1 represents player 1 guesses and guess 2 represents player 2 attepted guesses
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
    pub fn make_turn(&mut self, guess: &Guess) {
        match self.turn {
            false => {
                let idx: u64 = ((guess.y * 8) + guess.x).into();
            }
            true => {}
        };
    }

    fn player_display(&self, who: bool){
        match who{
            false => {}
            true => {}
        }
    }
    // fn display(grid: u64){
    //     for line in self.cells.as_slice().chunks(self.width as usize){}
    // }
}

/// Represets a guess of where the other player placed their dot.
/// u8 was choses because you only have 64 possible spaces
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Guess {
    pub x: u8,
    pub y: u8,
}