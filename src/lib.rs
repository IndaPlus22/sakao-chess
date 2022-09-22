use std::fmt::Display;
use std::marker::Copy;
use std::{char, fmt, u64, u16};

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

        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: Self::init_board(),
            //...
        }
    }
    
    /// If the current game state is `InProgress` and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// TODO: this
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
    /// TODO: just minus 6(pawn value) right now and then add the new piece value(for queen 2)
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
    /// TODO: this
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        None
    }

    // initializes the first board
    fn init_board() -> [u16; 64] {
        let board: [u16; 64] = Self::load_fen_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        //DEBUG
        Self::print_board(board);
        
        print!("{}", Self::piece_int_to_str(board[Self::strpos_to_pos("E1")]));
        //ok
        board
    }

    // loads a fen string which represents where all pieces are on the board
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

                board[(8 * y + x) as usize] = p;
                x += 1;
                p = 0;
            }
        }

        board
    }

    //prints board only for debug
    fn print_board(board: [u16; 64]) {
        print!("\n");
        for y in 0..8 {
            for x in 0..8 {
                print!(
                    "{}  ",
                    Self::piece_int_to_str(board[y * 8 + x]),
                );
            }
            print!("\n");
        }
    }

    // dont understand much here with masking bytes from https://github.com/SebLague/Chess-AI/blob/main/Assets/Scripts/Core/Piece.cs
    // gets role by masking
    fn get_role(piece: u16) -> u16 {
        piece & 0b00111 // typemask, takes away the two first bytes(which represents the color)
    }

    // gets color by masking 
    fn get_color(piece: u16) -> u16 {
        piece & (0b01000 | 0b10000) // colormask
    }
    //----------------------------------------------------------------

    // converts piece to which role and which color in char, lowercase is white and upper is black
    fn piece_int_to_str(piece: u16) -> char {
        if Self::get_color(piece) == BLACK {
            return Self::role_int_to_str(piece).to_uppercase().collect::<Vec<char>>()[0];
        }
        Self::role_int_to_str(piece)
    }

    // converts piece to which role it has in char
    fn role_int_to_str(piece: u16) -> char {
        match Self::get_role(piece) {
            KING =>   'k',
            QUEEN =>  'q',
            BISHOP => 'b',
            KNIGHT => 'n',
            ROOK =>   'r',
            PAWN =>   'p',
            _ => '.'
        }
    }

    // Converts int to file,rank e.g. E4 to (8 - 4) * 8 + E:(4) 
    fn strpos_to_pos(_postion: &str) -> usize {
        let rows = ["A", "B", "C", "D", "E", "F", "G", "H"];
        let char_vec: Vec<char> = _postion.chars().collect();
        let alph_index = rows.iter().position(|&r| r == char_vec[0].to_string()).unwrap();
        
        ((8 - char_vec[1].to_digit(10).unwrap()) as usize) * 8 + alph_index
    }

}

/// Implement print routine for Game.
///
/// Output example:
/// |                           |
/// | R  Kn B  Q  K  B  Kn R  8 |
/// | P  P  P  P  P  P  P  P  7 |
/// | *  *  *  *  *  *  *  *  6 |
/// | *  *  *  *  *  *  *  *  5 |
/// | *  *  *  *  *  *  *  *  4 |
/// | *  *  *  *  *  *  *  *  3 |
/// | P  P  P  P  P  P  P  P  2 |
/// | R  Kn B  Q  K  B  Kn R  1 |
/// | A  B  C  D  E  F  G  H
/// 
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
