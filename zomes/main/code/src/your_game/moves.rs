use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

/**
 *
 * The MoveType enum defines all the types of moves that are valid in your game and the
 * data they carry. In Checkers you can move a piece (MovePiece) from a location to another location.
 *
 */

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum MoveType {
    // <<DEVCAMP-TODO>> YOUR MOVE ENUM VARIENTS HERE
    MoveToken{x: usize, y: usize, distance: usize},
    CreateToken{distance: usize},
    // HomeToken{}?
}

impl MoveType {
	pub fn describe() -> Vec<MoveType> {
		// SHOULD RETURN AN EXAMPLE OF EACH VARIENT
		vec![MoveType::MoveToken{x: 3, y: 0, distance: 2},
             MoveType::CreateToken{x: 2, y: 0},]
	}
}
