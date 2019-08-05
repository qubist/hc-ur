# hc-ur

[![GitHub license](https://img.shields.io/github/license/qubist/hc-ur.svg)](https://github.com/qubist/hc-ur/blob/master/LICENSE.txt)

> The Ancient Mesopotamian Royal Game of Ur written as a simple Holochain app

## Background

Ur is an ancient board game. See [this video](https://www.youtube.com/watch?v=WZskjLq040I).

## The game
### Board setup

The game is played on a board consisting of an 8 x 3 grid with four squares missing at the following positions: (4,0),(5,0),(4,2),(5,2). Positions (0,0),(0,2),(3,1),(6,0), and (6,2) have "rosettes", decorative flowers, on them which affect gameplay. The board looks like this:

| [\*] | [ ] | [ ] | [ ]  |     |     | [\*] | [ ] |
|------|-----|-----|------|-----|-----|------|-----|
| [ ]  | [ ] | [ ] | [\*] | [ ] | [ ] | [ ]  | [ ] |
| [\*] | [ ] | [ ] | [ ]  |     |     | [\*] | [ ] |

From now on, let's denote it like this, with curly-brackets representing positions with rosettes.

| { } | [ ] | [ ] | [ ] |     |     | { } | [ ] |
|-----|-----|-----|-----|-----|-----|-----|-----|
| [ ] | [ ] | [ ] | { } | [ ] | [ ] | [ ] | [ ] |
| { } | [ ] | [ ] | [ ] |     |     | { } | [ ] |

### Game rules

*This is a condensed version of the rules suitible for describing the game for implementation to Holochain, but not historically accurate. For details, check the [Wikipedia page](https://en.wikipedia.org/wiki/Royal_Game_of_Ur) or play the game online at [YourTurnMyTurn](https://www.yourturnmyturn.com/java/ur/index.php) [WARNING: FLASH].*

* Each player has 7 of their tokens.
* Players alternate turns.
* On their turn the player flips four coins. The number of heads shown is how far they may move one of their token, including moving a token onto the board.
* Tokens move in this pattern:

| { ⇩ }  | [ ⇦ ]  | [ ⇦ ]  | [ ⇦ ]  |      |      | { ⇦ }  | [ ⇦ ]  |
|--------|--------|--------|--------|------|------|--------|--------|
| [⇨→]   | [⇨→]   | [⇨→]   | {⇨→}   | [⇨→] | [⇨→] | [⇨→]   | [⇧↓]   |
| { ↑ }  | [ ← ]  | [ ← ]  | [ ← ]  |      |      | { ← }  | [ ← ]  |

* Tokens can't move on top of tokens of the same type.
* A move may capture an opponent's token if the moved token lands on the opponent's token, except if the opponent's token is on a rosette.
* A captured token is bumped off the board.
* If a player moves a token to a rosette, they take an extra turn.
* If a player moves a token off the end of the path described above, that token is home but the token must exit exactly. (If I move to square (0,6) I then must roll a 1 to move my token off the board.)
* The objective of the game is to get all one's tokens home before one's opponent. The first player with all their tokens home wins.

## Implementation of the game to Holochain

### Moves

Moving a token already on the board:
```javascript
{
    game: "QmHashOfGame123",
    author: "QmMyAgentAddress000",
    previous_move: "QmHashOfPreviousMove"
    move_type: {
        MoveToken: {
            from: {x: 0, y: 2},
            distance: 4
        }
    }
}
```
Moving a token onto the board:
```javascript
{
    game: "QmHashOfGame123",
    author: "QmMyAgentAddress000",
    previous_move: "QmHashOfPreviousMove"
    move_type: {
        CreateToken: {
            distance: 3
        }
    }
}
```

### A player makes a valid move when:

* **CreateToken**
  * the game is not over
  * it is the player's turn *(calculate from who was the last player to move and whether they landed on a rosette)*
  * the move length is no more than 4 tiles
  * the move destination is not on top of another of the player's tokens *(calculate by reducing game state)*
  * the move length matches the player's roll \* I will not implement this to start with
  * *the player is not out of tokens* (tokens on board + tokens home < 7)
* **MoveToken**
  * the game is not over
  * it is the player's turn
  * the move length is no more than 4 tiles
  * the move destination is not on top of another of the player's tokens
  * the move length matches the player's roll
  * *a token belonging to the player exists at the from coordinates of the move*
  * *the move is no more than one tile off the end of the board*

### Game state

```javascript
{
    complete: false,
    player_1: {
        tokens: [{x: 4, y: 1}, {x: 2, y: 2}, {x: 6, y: 1}],
        tokens_home: 1,
        resigned: false,
    }
    player_2: {
        tokens: [{x: 2, y: 0}, {x: 5, y: 1}],
        tokens_home: 4,
        resigned: false,
    }
}
```
(to calculate tokens remaining off the board: `7 - [tokens_home + length(tokens)]`)

Reducing a sequence of moves into a game state:

* Keep track of tiles entering the board and moving and update their positions
* Watch tiles leaving the board and add them to the home tally
* Keep track of turns and double-turns
