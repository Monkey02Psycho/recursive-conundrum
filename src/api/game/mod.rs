use std::collections::HashMap;

use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    tokio::sync::RwLock,
    State,
};
/// The main api for the game. handles game logic and server request.
/// The idea behind a game is its a 8x8 grid and you have two bytes left over. move onto another piece to capture it.

#[derive(Debug, Default)]
pub struct Games(HashMap<String, Game>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ID(String);

/// a game is a 8 by 8 board. that leaves two bits for other information. that info who is who.
/// TODO how do we want game to work?
///
#[derive(Debug, Default)]
pub struct Game {
    /// the most recent match.
    board: u128,
    /// a history of the game. that does not include the most recent match.
    history: Vec<u128>,
}

/// Starts a new game. returns the ID of said game
#[post("/new_game")]
pub async fn new_game(games: &State<RwLock<Games>>) -> Json<ID> {
    let mut games = games.write().await;
    //TODO add way to increment ID by 1.
    games.0.insert(0.to_string(), Game::default());
    Json(ID(0.to_string()))
}

/// this handles all the game play. I will haveto make it multiplayer soon.
pub mod game_play {
    use std::marker::PhantomData;

    use rocket::{request::FromParam, serde::{json::Json, Deserialize}, tokio::sync::RwLock, State};

    use crate::api::game::Games;

    /// the multi million dollar question...
    /// how do we tell whos who. we cant trust the player to say.
    /// but we will for now.
    #[derive(Debug)]
    enum Player {
        Host,
        Guest,
    }

    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub enum Direction {
        left,
        right,
        up,
        down,
    }

    #[get("/update/<id>", format = "json", data = "<direction>")]
    pub async fn update(id: String, direction: Json<Direction>, games: &State<RwLock<Games>>) -> Json<u128> {
        // let id = ID(id);
        let mut games = games.write().await;

        // lets just assume for 30 seconds it doesnt panic
        let game = games.0.get_mut(&id).unwrap();
        
        Json(0)
    }
}
