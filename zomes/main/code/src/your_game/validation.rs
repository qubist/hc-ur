
use crate::game::Game;
use crate::game_move::Move;
use super::{
    GameState,
};
use hdk::holochain_persistence_api::cas::content::Address;
use super::state::Token;
use super::moves::MoveType;

/**
 *
 * To implement your own custom rule validation all you need to do is re-implement the function `is_valid` on `Move`
 *
 * This function  takes the current game and the game state (which includes all the existing moves)
 * and determines if a new candidate move is valid. Typically this will involve first matching on the move type
 * and then determining if the move is valid.
 *
 * It function must return Ok(()) if a move is valid and Err("Some error string") for an invalid move.
 * It is useful to provide descriptive error strings as these can be visible to the end user.
 *
 */


impl Move {
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        // Check if a move is valid given the current game and its state

        // it is the player's turn (calculate from who was the last player to move and whether they
        // landed on a rosette)
        is_players_turn(self.author.clone(), &game, &game_state)?;

        // the move length is no more than 4 tiles
        is_valid_distance(self.distance)?;

        // find destination of move
        if self.author == game.player_1 {
            let p = 1;
        } else {
            let p = 2;
        }
        let destination = game_state.incrementLocation(self.x, self.y, self.distance, p);

        // the move destination is not on top of another of the player's tokens (calculate by
        // reducing game state)
        player_can_move_to_tile(self.author.clone(), &game, &game_state, destination)?;

        match self.move_type {
            MoveType::CreateToken => {
                // the player is not out of tokens (tokens on board + tokens home < 7)
                player_has_token(self.author.clone(), &game, &game_state)?;
            },
            MoveType::MoveToken => {
                // a token belonging to the player exists at the from coordinates of the move
                token_exists(self.author.clone(), &game, &game_state, (self.x, self.y))?;

                // the move is no more than one tile off the end of the board
                isnt_overmoving((self.x, self.y), self.distance)?;
            },
        }
        
        Ok(())
    }
}

fn is_players_turn(player: Address, game: &Game, game_state: &GameState) -> Result<(), String> {
    let moves = &game_state.moves;
    match moves.last() {
        Some(last_move) => {
            // figure out what the location the token landed is
            // there are two ways to do this because of the two move types
            match last_move.move_type {
                MoveType::MoveToken => {
                    // player doesn't matter when calculating whether token is on rosette
                    let location = game_state.incrementLocation(last_move.x, last_move.y, last_move.distance, 1);
                },
                MoveType::CreateToken => {
                    let location = (last_move.x, last_move.y);
                },
            }

            if is_rosette(location) {
                if last_move.author == player {
                    // our player landed on rosette and plays again
                    Ok(())
                } else {
                    // player's opponend landed on rosette and plays again
                    Err("It is not your turn! ROSETTE".into())
                }
            } else {
                // no rosette
                if last_move.author == player {
                    // our player just moved, opponent's turn
                    Err("It is not your turn! NON-ROSETTE".into())
                } else {
                    // opponent just moved, our player's turn
                    Ok(())
                }
            }

        },
        None => {
            // no player has move yet, it is both players' turn !!
            Ok(())
            // // Willem's way: player 2 has to start since they're accepting the invitation
            // if game.player_2 == player {
            //     Ok(())
            // } else {
            //     Err("Player 2 must start".into())
            // }
        }
    }
}

fn player_can_move_to_tile(player: Address, game: &Game, game_state: &GameState, destination: (usize, usize)) -> Result<(), String> {
    match player {
        game.player_1 => {
            // if player 1 is sending a token to a place where they already have a token
            if game_state.p1_tokens.contains(Token{destination[0], destination[1]}) {
                Err("You can't move a token onto another of your tokens! P1".into())
            } else {
                Ok(())
            }
        },
        game.player_2 => {
            if game_state.p2_tokens.contains(Token{destination[0], destination[1]}) {
                Err("You can't move a token onto another of your tokens! P2".into())
            } else {
                Ok(())
            }
        },
    }
}

fn player_has_token(player: Address, game: &Game, game_state: &GameState) -> Result<(), String> {
    match player {
        game.player_1 => {
            if game_state.p1_tokens.len() + game_state.p1_home < 7 {
                Ok(())
            } else {
                Err("You are out of tokens! P1".into())
            }
        },
        game.player_2 => {
            if game_state.p2_tokens.len() + game_state.p2_home < 7 {
                Ok(())
            } else {
                Err("You are out of tokens! P2".into())
            }
        },
    }
}

fn token_exists(player: Address, game: &Game, game_state: &GameState, origin: (usize, usize)) -> Result<(), String> {
    match player {
        game.player_1 => {
            if game_state.p1_tokens.contains(Token{origin[0], origin[1]}) {
                Ok(())
            } else {
                Err("There is not one of your tokens to move on the selected tile P1".into())
            }
        },
        game.player_2 => {
            if game_state.p2_tokens.contains(Token{origin[0], origin[1]}) {
                Ok(())
            } else {
                Err("There is not one of your tokens to move on the selected tile P2".into())
            }
        },
    }
}

fn isnt_overmoving(origin: (usize, usize), distance: usize) -> Result<(), String> {
    // split up by tiles of origin of the move
    match origin {
        (7,1) => {
            // distance can't be 4
            if distance > 3 {
                Err("You must move off the board exactly! V3".into())
            } else {
                Ok(())
            }
        },
        (7,0|2) => {
            // distance can't be 3 or 4
            if distance > 2 {
                Err("You must move off the board exactly! V2".into())
            } else {
                Ok(())
            }
        },
        (6,0|2) => {
            // distance can't be 2, 3 or 4 (must be 1)
            if distance > 1 {
                Err("You must move off the board exactly! V1".into())
            } else {
                Ok(())
            }
        },
        (_,_) => {
            // from no other tiles can a 4-tile move end up more than one tile off the board
            Ok(())
        },
    }
}

fn is_rosette(x: usize, y: usize) -> bool {
    match (x,y) {
        (0,0) | (0,2) | (3,1) | (6,0) | (6,2) => return true,
        (_,_) => return false,
    }
}

fn is_valid_distance(d: usize) -> Result<(), String> {
    if d >= 4 {
        Err("You can't move more than 4 tiles!".into())
    } else {
        Ok(())
    }
}
