# sakao-chess

| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with pieces. |
| `pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game. |
| `pub fn set_promotion(&mut self, _piece: &str) -> ()` | Set the piece type that a pawn becames following a promotion. |
| `pub fn get_game_state(&self) -> GameState` | Get the current game state. |
| `pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>>` | If a piece is standing on the given tile, return all possible new positions of that piece.  |
| `pub fn turn_color(&self) -> u8` | return which color turn it is right now in form of White = 8 and Black = 16 |
| `pub fn check_if_checkmate(&self, _color: u8) -> bool` | checks if _color is being checkmated |

Positions are given as strings with the format `"<file><rank>"`.

Program also exports an enumerators `GameState` with the values:
- `InProgress`, 
- `Check`,
- `GameOver`, 

Program has consts for color and role of piece
`pub const NONE: u8 = 0`
`pub const KING: u8 = 1`
`pub const QUEEN: u8 = 2`
`pub const BISHOP: u8 = 3`
`pub const KNIGHT: u8 = 4`
`pub const ROOK: u8 = 5`
`pub const PAWN: u8 = 6`
`pub const WHITE: u8 = 8`
`pub const BLACK: u8 = 16`
