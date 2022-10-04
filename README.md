# sakao-chess

| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with pieces. |
| `pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game. |
| `pub fn set_promotion(&mut self, _piece: &str) -> ()` | Set the piece type that a pawn becames following a promotion. |
| `pub fn get_game_state(&self) -> GameState` | Get the current game state. |
| `pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>>` | If a piece is standing on the given tile, return all possible new positions of that piece.  |
| `pub fn turn_color(&self) -> u8` | return which color turn it is right now in form of White = 8 and Black = 16 |

Positions are given as strings with the format `"<file><rank>"`.

Your program also exports an enumerators `GameState` with the values:
- `InProgress`, 
- `Check`,
- `GameOver`, 
