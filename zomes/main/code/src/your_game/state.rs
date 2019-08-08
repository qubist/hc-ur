use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

use crate::game_move::Move;
use crate::game::Game;
use super::MoveType;


/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 *
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub p1_tokens: Vec<Token>,
    pub p1_home: usize,
    pub p2_tokens: Vec<Token>,
    pub p2_home: usize,
    // Implement your own game state
    // May be helpful to split this into state for each player
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Token {
    pub x: usize,
    pub y: usize,
}

impl GameState {
    pub fn initial() -> Self {
        // return an initial state of a game
        Self{
            moves: Vec::new(),
            p1_tokens: Vec::new(),
            p1_home: 0,
            p2_tokens: Vec::new(),
            p2_home: 0,
        }
    }

    pub fn render(&self) -> String {
        // <<DEVCAMP>> return a pretty formatting string representation
        "".to_string()
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> GameState {
        // <<DEVCAMP>>
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid

        let mut moves = self.moves.clone();
        let mut p1_tokens = self.p1_tokens.clone();
        let mut p1_home = self.p1_home.clone();
        let mut p2_tokens = self.p2_tokens.clone();
        let mut p2_home = self.p2_home.clone();

        // add new move to the list of all moves
        moves.push(next_move.clone());

        // update state according to move
        match next_move.move_type {
            MoveType::MoveToken{x, y, distance} => {
                if is_homing(x, y, distance) {
                    // if token is going home
                    if game.player_1 == next_move.author {
                        p1_home += 1;
                    } else {
                        p2_home += 1;
                    }
                } else {
                    // else token is moving
                    if game.player_1 == next_move.author {
                        // remove token at old location
                        // p1_tokens.retain(|&token| (token.x, token.y) != (x,y));
                        p1_tokens.retain(|token| (token.x, token.y) != (x,y));
                        // add token at new location
                        let (new_x, new_y) = increment_location(x, y, distance, 1);
                        p1_tokens.push(Token{x: new_x, y: new_y});
                    } else {
                        // remove token at old location
                        // p2_tokens.retain(|&token| (token.x, token.y) != (x,y));
                        p2_tokens.retain(|token| (token.x, token.y) != (x,y));
                        // add token at new location
                        let (new_x, new_y) = increment_location(x, y, distance, 2);
                        p2_tokens.push(Token{x: new_x, y: new_y});
                    }
                }
            },
            MoveType::CreateToken{distance} => {
                if game.player_1 == next_move.author {
                    let dest = increment_location(4, 0, distance, 1);
                    p1_tokens.push(Token{x: dest.0, y: dest.1});
                } else {
                    let dest = increment_location(4, 2, distance, 2);
                    p2_tokens.push(Token{x: dest.0, y: dest.1}); // FIXME not sure if this does anything because it's inside a different block than p2_tokens declaration
                }
            }
        }

        GameState {
            moves,
            p1_tokens: p1_tokens,
            p1_home: p1_home,
            p2_tokens: p2_tokens,
            p2_home: p2_home,
        }
    }

}

// takes token location, distance, and player and returns the location where a token at the
// given location will land if moved the given distance by the given player.
pub fn increment_location(x: usize, y: usize, distance: usize, player: usize) -> (usize, usize) {

    let mut location = (x, y);

    fn step(x: usize, y: usize, player: usize) -> (usize, usize) {
        match (x, y) {
            // merge
            (0,0) | (0,2) => (0,1),
            //split
            (7,1) => {
                if player == 1 {
                    (7,0)
                } else {
                    (7,2)
                }
            },
            // middle row
            (_,1) => (x+1,y),
            // outside rows (only thing left)
            (_,_) => (x-1,y),
        }
    }

    for _n in 1..=distance {
        location = step(location.0, location.1, player);
    }

    return location;
}

// test whether a token moved a given distance from a given location will go home.
fn is_homing(x: usize, y: usize, distance: usize) -> bool {
    // player doesn't matter when calculating whether token homes
    let new_location = increment_location(x, y, distance, 1);
    return new_location == (5,0) || new_location == (5,2);
}
