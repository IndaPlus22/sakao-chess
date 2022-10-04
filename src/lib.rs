use std::marker::Copy;
use std::{char, fmt, u64, u8};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

struct Bitmap {
    pub val: u64,
}

impl Bitmap {
    pub fn new(value: u64) -> Bitmap {
        Bitmap { val: value }
    }

    // shift the bitmap to the left by n bits and check if the result is 0
    pub fn get(&self, position: u8) -> bool {
        return (self.val >> position) & 1 == 1;
    }

    pub fn set(&mut self, position: u8) -> () {
        self.val = self.val | 1 << position;
    }

    pub fn remove(&mut self, position: u8) -> () {
        self.val = self.val & !(1 << position);
    }
}

// const for pieces and not enums
pub const NONE: u8 = 0;
pub const KING: u8 = 1;
pub const QUEEN: u8 = 2;
pub const BISHOP: u8 = 3;
pub const KNIGHT: u8 = 4;
pub const ROOK: u8 = 5;
pub const PAWN: u8 = 6;

pub const WHITE: u8 = 8;
pub const BLACK: u8 = 16;

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: [u8; 64],
    turn: u8,
    //...
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: Self::init_board(),
            turn: 0,
        }
    }

    /// If the current game state is `InProgress` and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// TODO: doesnt check chek
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        if self.state != GameState::InProgress {
            None //TODO:
        } else {
            if _from.len() == 2 && _to.len() == 2 { // if from and to are valid
                let piece = self.board[Self::pos_str_to_int(_from)];
                if piece != 0 && Self::get_color(piece) == self.turn_color() { // if piece is white and its whites turn and piece is not none
                    let moves = self.get_possible_moves(_from);

                    if moves.is_none() {
                        return Some(self.state);
                    } else {
                        if moves.unwrap().contains(&_to.to_string()) {
                            // move piece
                            let from = Self::pos_str_to_int(_from) as u8;
                            let to = Self::pos_str_to_int(_to) as u8;
                            self.board = self.do_move(from, to);

                            if self.check_if_checked(Self::get_color(to)) {
                                self.state = GameState::Check;
                            }

                            self.turn += 1;

                            Self::print_board(self.board);
                            println!("turn: {}, total moves {}", self.turn_color(), self.turn);
                            
                            return Some(self.state);
                        } else {
                            return None;
                        }
                    }
                }
            }

            // check if move is possible

            Some(GameState::InProgress)
        }
    }

    /// Set the piece type that a pawn becomes following a promotion.
    /// TODO: just minus 6(pawn value) right now and then add the new piece value(for queen 2)
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        let piece_int: u8 = self::Game::piece_str_to_int(_piece);
    }

    /// Get the current game state.
    ///
    ///hello
    ///# HELLO
    ///ok
    ///
    ///haha
    ///
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&mut self, _position: &str) -> Option<Vec<String>> {
        // not a valid pos return None
        if _position.len() != 2 {
            return None;
        } else {
            let pos_int: u8 = Self::pos_str_to_int(_position) as u8;
            let mut moves: Vec<u8> = self.get_possible_moves_int(pos_int);
            let color = Self::get_color(self.board[pos_int as usize]);

            // check if king is in check
            moves = self.filter_checked_moves(moves, pos_int);

            // Placement
            let mut moves_string: Option<Vec<String>> = Some(self.posvec_int_to_string(moves));
            return moves_string;
        }
    }

    fn turn_color(&self) -> u8 {
        if self.turn % 2 == 0 {
            return 8;
        } else {
            return 16;
        }
    }

    // returns a bitmap with all attacking movees
    fn gen_bitmap_attacked(&self, _color: u8) -> Bitmap {
        let mut bitmap: Bitmap = Bitmap::new(0);

        for i in 0..64 {
            if Self::get_color(self.board[i]) == _color {
                // if color is right
                let moves = self.get_possible_moves_int(i as u8); // get all possible moves
                for m in moves {
                    // set all positions that are attacked to 1
                    bitmap.set(m);
                    // println!("attacking spot from: {} to: {}", Self::pos_int_to_string(i as u8), Self::pos_int_to_string(m as u8));
                    // println!("{}", bitmap.val);
                }
            }
        }

        bitmap
    }

    fn get_possible_moves_int(&self, _position: u8) -> Vec<u8> {
        let role: u8 = Self::get_role(self.board[_position as usize]);

        let mut moves: Vec<u8> = Vec::new();
        match role {
            PAWN => {
                // can take but no en passant
                moves = self.get_pawn_moves(_position);
            }
            KING => {
                moves = self.get_king_moves(_position);
            }
            QUEEN => {
                let mut a = self.get_diagonal_moves(_position);
                let mut b = self.get_orthogonal_moves(_position);

                a.append(&mut b);
                moves = a;
            }
            ROOK => {
                moves = self.get_orthogonal_moves(_position);
            }
            BISHOP => {
                moves = self.get_diagonal_moves(_position);
            }
            KNIGHT => {
                moves = self.get_knight_moves(_position);
            }
            _ => {}
        }

        moves
    }

    fn filter_checked_moves(&mut self, _moves: Vec<u8>, _from: u8) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();
        let mut board = self.board;
        let color = Self::get_color(board[_from as usize]);
        let bitmap = self.gen_bitmap_attacked(color ^ 0b11000);

        // do every move and check if king is attacked
        for m in _moves {
            self.board = self.do_move(_from, m);

            //++++++++++++++++++++dasdasdasd
            print!("\n");

            Self::print_board(board);

            print!("\n");
            for y in 0..8 {
                for x in 0..8 {
                    print!(
                        "{}{} ",
                        bitmap.get(y * 8 + x) as u8,
                        Self::piece_int_to_str(self.board[(y * 8 + x) as usize])
                    );
                }
                print!("\n");
            }
            print!("\n");
            //++++++++++++++++++++dasdasdasd

            if !self.check_if_checked(color) {
                // when checkifchecked is 0
                moves.push(m);
            } else {
                println!("checked::::::: {}", Self::pos_int_to_string(m));
            }
            println!(
                "fromto{} {}",
                Self::pos_int_to_string(_from),
                Self::pos_int_to_string(m)
            );
            self.board = board;
        }

        //++++++++++++++++++++dasdasdasd
        print!("\n");

        Self::print_board(board);

        print!("\n");
        for y in 0..8 {
            for x in 0..8 {
                print!(
                    "{}{} ",
                    bitmap.get(y * 8 + x) as u8,
                    Self::piece_int_to_str(self.board[(y * 8 + x) as usize])
                );
            }
            print!("\n");
        }
        print!("\n");
        //++++++++++++++++++++dasdasdasd

        moves
    }

    fn do_move(&self, _from: u8, _to: u8) -> [u8; 64] {
        let mut tboard = self.board;
        tboard[_to as usize] = tboard[_from as usize];
        tboard[_from as usize] = 0;

        tboard
    }
    /// get all possiblemoves from the attacking side and check if there is a king on any of the moves
    /// if there is a king with attacked color return true
    /// else return false
    /// TODO: redesign
    ///
    fn check_if_checked(&self, _attacked_color: u8) -> bool {
        let attacking_color = _attacked_color ^ 0b11000; // can be !attacked_color & 0b11000
                                                         // 01000  10000
                                                         // 11000  11000
                                                         // 10000  01000
        let bitmap = self.gen_bitmap_attacked(attacking_color);

        // println!("attacked color: {}", _attacked_color);
        // println!("attacking color: {}", attacking_color);
        // kingpos
        let mut kingpos = 0;
        for i in 0..64 {
            if _attacked_color == Self::get_color(self.board[i])
                && KING == Self::get_role(self.board[i])
            {
                kingpos = i;
                // println!("Found king");
                break;
            }
        }
        println!(
            "kingpos: {}, color: {}",
            kingpos,
            Self::get_color(self.board[kingpos])
        );

        let mut kingposbitmap: Bitmap = Bitmap::new(0);
        kingposbitmap.set(kingpos as u8);

        // println!("kingposbitmap {}", kingposbitmap.val);
        // println!("bittt {}", bitmap.val);
        // println!("kingposbitmap & bitmap {}", kingposbitmap.val & bitmap.val);
        (bitmap.val & kingposbitmap.val) != 0

        //  1110101
        //  0100000
        //  0100000
        //  inte noll true

        //  1100011
        //  0001000
        //  0000000
        //  noll false inte check
    }

    fn get_moves_for_color(&self, _color: u8) -> Vec<u8> {
        let mut total_moves: Vec<u8> = Vec::new();
        for i in 0..64 {
            if Self::get_color(self.board[i]) == _color {
                total_moves.append(&mut self.get_possible_moves_int(i as u8));
            }
        }

        total_moves
    }

    fn posvec_int_to_string(&self, moves: Vec<u8>) -> Vec<String> {
        let mut moves_string: Vec<String> = Vec::new();
        for i in moves {
            moves_string.push(Self::pos_int_to_string(i));
        }

        moves_string
    }

    // rokad :(
    fn get_king_moves(&self, _position: u8) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();
        // filter all moves that make you checked
        let color: u8 = Self::get_color(self.board[_position as usize]);

        let direction: [i16; 8] = [-8, 1, 8, -1, -7, 9, 7, -9];

        for i in 0..8 {
            let destination = _position as i16 + direction[i];
            if destination >= 0 && destination < 64 {
                if Self::get_color(self.board[destination as usize]) != color {
                    moves.push(destination as u8);
                }
            }
        }
        moves
    }

    fn get_knight_moves(&self, _position: u8) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();

        let direction: [i16; 8] = [-15, -6, 10, 17, 15, 6, -10, -17];

        for i in 0..8 {
            let destination = _position as i16 + direction[i];
            if destination >= 0 && destination < 64 {
                if Self::get_color(self.board[destination as usize])
                    != Self::get_color(self.board[_position as usize])
                {
                    moves.push(destination as u8);
                }
            }
        }

        moves
    }

    //TODO: (if i want en passant)
    fn get_pawn_moves(&self, _position: u8) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();

        let mut color_mult: i16 = 1;

        if Self::get_color(self.board[_position as usize]) == WHITE {
            color_mult = -1;
        }

        let sides: [i16; 2] = [9 * color_mult, 7 * color_mult];
        for i in sides {
            let next_pos = self.board[(_position as i16 + i) as usize];
            if next_pos != 0
                && (Self::get_color(next_pos)) != Self::get_color(self.board[_position as usize])
            {
                moves.push((_position as i16 + i) as u8);
            }
        }
        if self.board[(_position as i16 + 8 * color_mult) as usize] == 0 {
            moves.push((_position as i16 + 8 * color_mult) as u8); // go upp/down one step

            if _position / 8 == 6
                && color_mult == -1
                && self.board[(_position as i16 + 8 * color_mult * 2) as usize] == 0
            {
                // has not moved yet as white
                moves.push(_position - (8 * 2));
            } else if _position / 8 == 1
                && self.board[(_position as i16 + 8 * color_mult * 2) as usize] == 0
            {
                // has not moved yet as black
                moves.push(_position + (8 * 2));
            }
        }

        moves
    }

    fn get_orthogonal_moves(&self, _position: u8) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();

        let direction: [i16; 4] = [-8, 1, 8, -1];
        let count = [
            _position / 8,
            7 - (_position % 8),
            7 - _position / 8,
            _position % 8,
        ];

        for i in 0..4 {
            for j in 1..(count[i] + 1) {
                let tmp_pos = (_position as i16 + direction[i] * j as i16) as usize;
                if self.board[tmp_pos] != 0 {
                    // if there is a piece on the way
                    if Self::get_color(self.board[tmp_pos])
                        != Self::get_color(self.board[_position as usize])
                    {
                        // if it is enemy be able to take it
                        moves.push(tmp_pos as u8);
                    }
                    break;
                }
                moves.push(tmp_pos as u8);
            }
        }

        moves
    }

    fn get_diagonal_moves(&self, _position: u8) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();

        let direction: [i16; 4] = [-7, 9, 7, -9];
        let count = [
            std::cmp::min(_position / 8, 7 - _position % 8),
            std::cmp::min(7 - _position / 8, 7 - _position % 8),
            std::cmp::min(7 - _position / 8, _position % 8),
            std::cmp::min(_position / 8, _position % 8),
        ];

        for i in 0..4 {
            for j in 1..(count[i] + 1) {
                let tmp_pos = (_position as i16 + direction[i] * j as i16) as usize;
                if self.board[tmp_pos] != 0 {
                    // if there is a piece on the way
                    if Self::get_color(self.board[tmp_pos])
                        != Self::get_color(self.board[_position as usize])
                    {
                        // if it is enemy be able to take it
                        moves.push(tmp_pos as u8);
                    }
                    break;
                }
                moves.push(tmp_pos as u8);
            }
        }

        moves
    }

    // initializes the first board
    fn init_board() -> [u8; 64] {
        let board: [u8; 64] = Self::load_fen_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        // testing
        // let board: [u8; 64] = Self::load_fen_board("rnb1kbnr/pppppppp/8/8/1q6/8/PPPPPPPP/RNBQKBNR");

        //DEBUG
        Self::print_board(board);

        print!(
            "{}",
            Self::piece_int_to_str(board[Self::pos_str_to_int("E1")])
        );
        //ok

        board
    }

    // loads a fen string which represents where all pieces are on the board
    fn load_fen_board(fen: &str) -> [u8; 64] {
        let mut board: [u8; 64] = [0; 64];

        let mut x: u8 = 0;
        let mut y: u8 = 0;
        let mut p: u8 = 0;

        for item in fen.chars() {
            if item == '/' {
                y += 1;
                x = 0;
            } else if item.is_numeric() {
                x += item.to_digit(10).unwrap() as u8;
            } else {
                if item.is_uppercase() {
                    p += WHITE;
                } else {
                    p += BLACK;
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
    fn print_board(board: [u8; 64]) {
        let rows = ["A", "B", "C", "D", "E", "F", "G", "H"];

        print!("\n");
        for y in 0..8 {
            print!("{} | ", 8 - y);
            for x in 0..8 {
                print!("{}  ", Self::piece_int_to_str(board[y * 8 + x]),);
            }
            print!("\n");
        }
        println!("    ----------------------");
        print!("    ");
        for i in 0..8 {
            print!("{}  ", rows[i]);
        }
        print!("\n");
    }

    // dont understand much here with masking bytes from https://github.com/SebLague/Chess-AI/blob/main/Assets/Scripts/Core/Piece.cs
    // gets role by masking
    fn get_role(piece: u8) -> u8 {
        piece & 0b00111 // typemask, takes away the two first bytes(which represents the color)
                        //      e.g.
                        //      0b01001 white king
                        //      0b00111 mask
                        //    & 0b00001 king with no color
    }

    // gets color by masking
    fn get_color(piece: u8) -> u8 {
        piece & 0b11000 // colormask
    }
    //----------------------------------------------------------------

    // converts position to index in array dont know if needed.
    fn piece_str_to_int(piece: &str) -> u8 {
        let mut p: u8 = 0;

        if piece.chars().nth(0).unwrap().is_uppercase() {
            p += BLACK;
        } else {
            p += WHITE;
        }

        let role_string = piece.to_lowercase().to_string();
        match role_string.as_str() {
            "k" => p += KING,
            "q" => p += QUEEN,
            "b" => p += BISHOP,
            "n" => p += KNIGHT,
            "r" => p += ROOK,
            "p" => p += PAWN,
            _ => print!("ERROR"),
        }

        p
    }

    // converts piece to which role and which color in char, lowercase is white and upper is black
    fn piece_int_to_str(piece: u8) -> char {
        if Self::get_color(piece) == BLACK {
            return Self::role_int_to_str(piece)
                .to_uppercase()
                .collect::<Vec<char>>()[0];
        }
        Self::role_int_to_str(piece)
    }

    // converts piece to which role it has in char
    fn role_int_to_str(piece: u8) -> char {
        match Self::get_role(piece) {
            KING => 'k',
            QUEEN => 'q',
            BISHOP => 'b',
            KNIGHT => 'n',
            ROOK => 'r',
            PAWN => 'p',
            _ => '.',
        }
    }

    // Converts file,rank to int e.g. E4 to (8 - 4) * 8 + E:(4)
    fn pos_str_to_int(_postion: &str) -> usize {
        let rows = ["A", "B", "C", "D", "E", "F", "G", "H"];
        let char_vec: Vec<char> = _postion.chars().collect();
        let alph_index = rows
            .iter()
            .position(|&r| r == char_vec[0].to_string())
            .unwrap();

        ((8 - char_vec[1].to_digit(10).unwrap()) as usize) * 8 + alph_index
    }

    // 7 becomes G1
    fn pos_int_to_string(_position: u8) -> String {
        let rows = ["A", "B", "C", "D", "E", "F", "G", "H"];

        let mut file: String = String::from(rows[(_position % 8) as usize].to_string());
        let rank: char = char::from_digit(8 - (_position / 8) as u32, 10).unwrap();
        file.push(rank);

        file
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

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let mut game = Game::new();

        println!("\n-----------------------");

        // let _fuck = Game::piece_int_to_str(game.board[(8 * 0) + 3]);
        // print!("{}", _fuck);

        let ok = game.get_possible_moves("D2").unwrap();
        for i in ok {
            print!("\n");
            print!("a: {} ", i);
        }

        game.make_move("D2", "D4");

        print!("\n");

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
