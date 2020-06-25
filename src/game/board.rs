#[derive(Debug, Copy, Clone)]
pub enum Colour {
    Black,
    White,
    Empty
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    colour: Colour,
    character: char,
}

pub struct Board {
    // pieces: [((i8, i8), char, Colour); 64], //chess board 8*8   //char represents piece at the point
    //                                                     //(i8, i8) represents piece on the board
    pieces: [[Piece; 8]; 8],  
    white_score: i8,
    black_score: i8,
}

impl Board {

    pub fn new() -> Board { //h for horse

        //starting position
        let mut p = [[Piece{colour: Colour::Empty, character: '.'}; 8]; 8];
        p[0][0] = Piece{character:'c', colour:Colour::Black};     p[6][0] = Piece{character:'p', colour:Colour::White};
        p[0][1] = Piece{character:'h', colour:Colour::Black};     p[6][1] = Piece{character:'p', colour:Colour::White};
        p[0][2] = Piece{character:'b', colour:Colour::Black};     p[6][2] = Piece{character:'p', colour:Colour::White};
        p[0][3] = Piece{character:'q', colour:Colour::Black};     p[6][3] = Piece{character:'p', colour:Colour::White};
        p[0][4] = Piece{character:'k', colour:Colour::Black};     p[6][4] = Piece{character:'p', colour:Colour::White};
        p[0][5] = Piece{character:'b', colour:Colour::Black};     p[6][5] = Piece{character:'p', colour:Colour::White};
        p[0][6] = Piece{character:'h', colour:Colour::Black};     p[6][6] = Piece{character:'p', colour:Colour::White};
        p[0][7] = Piece{character:'c', colour:Colour::Black};     p[6][7] = Piece{character:'p', colour:Colour::White};
        p[1][0] = Piece{character:'p', colour:Colour::Black};     p[7][0] = Piece{character:'c', colour:Colour::White};
        p[1][1] = Piece{character:'p', colour:Colour::Black};     p[7][1] = Piece{character:'h', colour:Colour::White};
        p[1][2] = Piece{character:'p', colour:Colour::Black};     p[7][2] = Piece{character:'b', colour:Colour::White};
        p[1][3] = Piece{character:'p', colour:Colour::Black};     p[7][3] = Piece{character:'q', colour:Colour::White};
        p[1][4] = Piece{character:'p', colour:Colour::Black};     p[7][4] = Piece{character:'k', colour:Colour::White};
        p[1][5] = Piece{character:'p', colour:Colour::Black};     p[7][5] = Piece{character:'b', colour:Colour::White};
        p[1][6] = Piece{character:'p', colour:Colour::Black};     p[7][6] = Piece{character:'h', colour:Colour::White};
        p[1][7] = Piece{character:'p', colour:Colour::Black};     p[7][7] = Piece{character:'c', colour:Colour::White};

        Board {
            pieces: p,
            white_score: 0,
            black_score: 0,
        }
    }

    pub fn print_board(&self) {
        let mut line: i8 = 0;
        println!("  0 1 2 3 4 5 6 7");
        print!("0 ");
        for (i, row) in self.pieces.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                print!("{} ", col.character);
            }
            println!();
            line = line+1;
            if line != 8 { print!("{} ",line);}
        }
        println!();
    }

    pub fn make_move(&self, piece_pos: &String, target_pos: &String, turn: Colour) -> bool {
        //conver to (x, y);
        //checks that coordinates are valid numbers and on the board
        let piece_pos = match convert(piece_pos.trim()) {
            Some(coords) => coords,
            None => {println!("Bad piece position"); return false}
        };
        let target_pos = match convert(target_pos.trim()) {
            Some(coords) => coords,
            None => {println!("Bad target position"); return false}
        };

        if self.valid_move(&piece_pos, &target_pos, &turn) {
            println!("nice");
        }
        //check both positions on board first
        // if (on_board(piece_pos) && on_board(target_pos)) {

        // } else {
        //     println!("Position not on the board!");
        //     false
        // }
        true
    }

    pub fn valid_move(&self, piece_pos: &(usize, usize), target_pos: &(usize, usize), turn:&Colour) -> bool {

        if self.check_valid_piece(&piece_pos, &turn) {
             let moves = self.get_piece_moves(piece_pos);
        }
        false
        //iterate through pieces and find correct position
    }

    pub fn check_valid_piece(&self, piece_pos:  &(usize, usize), turn:&Colour) -> bool {
        //y is x and x is y :((
        let piece = self.pieces[piece_pos.1][piece_pos.0];
        println!("Piece is {}", piece.character);
        if piece.colour == turn {
            println!("THAT IS YOUR COLOUR WELL DONE");
            return true;
        } else if piece.colour == Colour::Empty {
            println!("Empty piece");
            return false;
        } else {
            println!("That is not your colour!");
            return false;
        }
    }

    pub fn get_piece_moves(&self, piece_pos: &(usize, usize)) -> &[(usize, usize)] {
        let piece = self.pieces[piece_pos.1][piece_pos.0];
        match piece.character {
            //each piece
            'p' => &[],
            'c' => &[],
            'h' => &[],
            'b' => &[],
            'k' => &[],
            'q' => &[],
        }
    }
}

fn convert(position: &str) -> Option<(usize, usize)> {
    let coords: Vec<&str> = position.split(",").collect();
    if coords.len() != 2 {
        return None    //poor formatting
    } else {
        let x: usize = match coords[0].trim().parse() {
            Ok(num) => if num < 8 {num} else {println!("Coord outside board");
                                                 return None},
            Err(_) => return None,  //not a number
        };
        let y: usize = match coords[1].trim().parse() {
            Ok(num) => if num < 8 {num} else {println!("Coord outside board");
                                                return None},
            Err(_) => return None,  //not a number
        };
        return Some((x, y))
    }
}
