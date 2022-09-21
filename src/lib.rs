use std::fmt::Display;
use std::marker::Copy;
use std::{char, fmt, u64};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

// const for pieces and not enums
pub const NONE: u16 = 0;
pub const KING: u16 = 1;
pub const QUEEN: u16 = 2;
pub const BISHOP: u16 = 3;
pub const KNIGHT: u16 = 4;
pub const ROOK: u16 = 5;
pub const PAWN: u16 = 6;

pub const WHITE: u16 = 8;
pub const BLACK: u16 = 16;

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: [u16; 64],
    //...
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let new_board: [u16; 64] = init_board();

        fn init_board() -> [u16; 64] {
            let mut board: [u16; 64] = load_fen_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
            

            print_board(board);
            board
        }

        fn load_fen_board(fen: &str) -> [u16; 64] {
            let mut board: [u16; 64] = [0; 64];

            let mut x: u16 = 0;
            let mut y: u16 = 7;
            let mut p: u16 = 0;

            for item in fen.chars() {
                if item == '/' {
                    y -= 1;
                    x = 0;
                } else if item.is_numeric() {
                    x += item.to_digit(10).unwrap() as u16;
                } else {
                    if item.is_uppercase() {
                        p += BLACK;
                    } else {
                        p += WHITE;
                    }

                    let role_string = item.to_lowercase().to_string();
                    match role_string.as_str() {
                        "k" => p += KING,
                        "q" => p += QUEEN,
                        "b" => p += BISHOP,
                        "n" => p += KNIGHT,
                        "r" => p += ROOK,
                        "p" => p += PAWN,
                        _ => print!("ERROR"),
                    }

                    x += 1;
                    board[(8 * y + x -1) as usize] = p;
                    p = 0;
                }
            }

            board
        }

        fn print_board(board: [u16; 64]) {
            print!("\n");
            for y in 0..8 {
                for x in 0..8 {
                    print!(
                        "{},{} ",
                        get_role(board[y * 8 + x]),
                        get_color(board[y * 8 + x])
                    );
                }
                print!("\n");
            }
        }

        // dont understand much here with masking bytes from https://github.com/SebLague/Chess-AI/blob/main/Assets/Scripts/Core/Piece.cs
        fn get_role(piece: u16) -> u16 {
            piece & 0b00111 // typemask, takes away the two first bytes(which represents the color)
        }

        fn get_color(piece: u16) -> u16 {
            piece & (0b01000 | 0b10000) // colormask
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
    fn strpos_to_pos(_postion: &str) -> (u8, u8) {
        let rows = ["A", "B", "C", "D", "E", "F", "G", "H"];

        (0, 0)
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
