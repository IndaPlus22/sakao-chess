use std::fmt::Display;
use std::marker::Copy;
use std::{char, fmt, u64};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}
#[derive(Debug, Copy, Clone)]
enum Color {
    White = 0,
    Black = 1,
    Empty = 2,
}

#[derive(Debug, Copy, Clone)]
enum Role {
    King = 0,
    Queen = 1,
    Bishop = 2,
    Knight = 3,
    Rook = 4,
    Pawn = 5,
    Empty = 9,
}

#[derive(Debug, Copy, Clone)]
struct Piece {
    role: Role,
    color: Color,
}

impl Piece {
    fn new(role: Role, color: Color) -> Piece {
        Piece {
            role: role,
            color: color,
        }
    }

    fn no_piece() -> Piece {
        Piece {
            role: Role::Empty,
            color: Color::Empty,
        }
    }

    fn get_role(&self) -> Role {
        self.role
    }

    fn get_color(&self) -> Color {
        self.color
    }
}
/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: [[Piece; 8]; 8],
    //...
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let new_board: [[Piece; 8]; 8] = init_board();

        fn init_board() -> [[Piece; 8]; 8] {
            let mut map: [[Piece; 8]; 8] = [[Piece::no_piece(); 8]; 8];

            for y in 0..map.len() {
                for x in 0..map[y].len() {
                    if y == 0 {
                        if x == 0 || x == 7 {
                            map[x][y] = Piece::new(Role::Rook, Color::Black);
                        } else if x == 1 || x == 6 {
                            map[x][y] = Piece::new(Role::Knight, Color::Black);
                        } else if x == 2 || x == 5 {
                            map[x][y] = Piece::new(Role::Bishop, Color::Black);
                        } else if x == 3 {
                            map[x][y] = Piece::new(Role::Queen, Color::Black);
                        } else {
                            map[x][y] = Piece::new(Role::King, Color::Black);
                        }
                    } else if y == 1 {
                        map[x][y] = Piece::new(Role::Pawn, Color::Black);
                    } else if y == 6 {
                        map[x][y] = Piece::new(Role::Pawn, Color::White);
                    } else if y == 7 {
                        if x == 0 || x == 7 {
                            map[x][y] = Piece::new(Role::Rook, Color::White);
                        } else if x == 1 || x == 6 {
                            map[x][y] = Piece::new(Role::Knight, Color::White);
                        } else if x == 2 || x == 5 {
                            map[x][y] = Piece::new(Role::Bishop, Color::White);
                        } else if x == 3 {
                            map[x][y] = Piece::new(Role::Queen, Color::White);
                        } else {
                            map[x][y] = Piece::new(Role::King, Color::White);
                        }
                    }
                }
            }

            print_board(map);
            map
        }

        fn print_board(map: [[Piece; 8]; 8]) {
            print!("\n");
            for y in 0..map.len() {
                for x in 0..map[y].len() {
                    print!("{}{} ", map[x][y].get_role() as u8, map[x][y].get_color() as u8);
                }
                print!("\n");
            }
        }

        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: new_board,
            //...
        }
    }

    /// If the current game state is `InProgress` and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        if self.state != GameState::InProgress {
            None
        } else {
            // check if move is possible
            // make the move
            // return gamestate
            Some(GameState::InProgress)
        }
    }

    /// Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {


        None
    }

    // TODO: Convert e.g. E4 to 4, 4
    fn strpos_to_pos (_postion: &str) -> (u8, u8) {
        let rows = ["A", "B", "C", "D", "E", "F", "G", "H"];

        (0,0)
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |    0  1  2  3  4  5  6  7    |
/// | 0  R  Kn B  Q  K  B  Kn R  8 |
/// | 1  P  P  P  P  P  P  P  P  7 |
/// | 2  *  *  *  *  *  *  *  *  6 |
/// | 3  *  *  *  *  *  *  *  *  5 |
/// | 4  *  *  *  *  *  *  *  *  4 |
/// | 5  *  *  *  *  *  *  *  *  3 |
/// | 6  P  P  P  P  P  P  P  P  2 |
/// | 7  R  Kn B  Q  K  B  Kn R  1 |
/// |    A  B  C  D  E  F  G  H
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
