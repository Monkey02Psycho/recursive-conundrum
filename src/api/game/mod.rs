use rocket::{serde::{json::Json, Deserialize, Serialize}, State, tokio::sync::RwLock};
/// The main api for the game. handles game logic and server request.
///


#[derive(Debug, Default)]
pub struct Games{
    game_ids: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ID(String);

/// a game is a 8 by 8 board. that leaves two bits for other information. that info who is who.
pub struct Game{
    /// the most recent match.
    board: u128,
    /// a history of the game. that does not include the most recent match.
    history: Vec<u128>
}

/// Starts a new game. returns the ID of said game
#[post("/new_game")]
pub async fn new_game(games: &State<RwLock<Games>>) -> Json<ID>{
    let mut games = games.write().await;
    //TODO add way to increment ID by 1.
    games.game_ids.push(1.to_string());
    Json(ID(games.game_ids.len().to_string()))
}

pub mod game_play {
    
}



