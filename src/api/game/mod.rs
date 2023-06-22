use guess_game::*;
use rocket::{serde::json::Json, tokio::sync::RwLock, State};

/// makes a new game. All with a default board.
#[get("/new", format = "json", data = "<guess>")]
fn new_game(guess: Json<Guess>, state: &State<RwLock<Games>>) -> Json<Game> {
    match state.try_write() {
        Ok(mut good) => {
            let mut game = Game::default();

            game.first_turn(&guess);

            good.0.insert(rand::random(), Game::default()).unwrap();
        }
        Err(_) => todo!(),
    };
    rocket::serde::json::Json(Game::default())
}

#[put("/turn/<id>", format = "json", data = "<guess>")]
fn turn(id: u128, guess: Json<Guess>, games: &State<RwLock<Games>>) -> Json<Game> {
    // should we let the client decide who that is or not
    match games.try_write() {
        Ok(mut game) => {
            let current = game.0.get_mut(&id).unwrap();
        }
        Err(_) => todo!(),
    };
    todo!()
}
