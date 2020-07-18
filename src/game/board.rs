
#[derive(Debug, Copy, Clone, PartialEq)]
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
    //2d array of 64 piece intances
    pieces: [[Piece; 8]; 8],  
    white_score: i8,
    black_score: i8,
}

impl Board {

    pub fn new() -> Board { //h for horse

        //starting position
        //hardcoded - could seperate into another files
        let mut p = [[Piece{colour: Colour::Empty, character: '.'}; 8]; 8];
        p[0][0] = Piece{character:'\u{265C}', colour:Colour::Black};     p[6][0] = Piece{character:'p', colour:Colour::White};
        p[0][1] = Piece{character:'h', colour:Colour::Black};     p[6][1] = Piece{character:'p', colour:Colour::White};
        p[0][2] = Piece{character:'b', colour:Colour::Black};     p[6][2] = Piece{character:'p', colour:Colour::White};
        p[0][3] = Piece{character:'q', colour:Colour::Black};     p[6][3] = Piece{character:'p', colour:Colour::White};
        p[0][4] = Piece{character:'k', colour:Colour::Black};     p[6][4] = Piece{character:'p', colour:Colour::White};
        p[0][5] = Piece{character:'b', colour:Colour::Black};     p[6][5] = Piece{character:'p', colour:Colour::White};
        p[0][6] = Piece{character:'h', colour:Colour::Black};     p[6][6] = Piece{character:'p', colour:Colour::White};
        p[0][7] = Piece{character:'c', colour:Colour::Black};     p[6][7] = Piece{character:'p', colour:Colour::White};
        p[1][0] = Piece{character:'p', colour:Colour::Black};     p[7][0] = Piece{character:'\u{2656}', colour:Colour::White};
        p[1][1] = Piece{character:'p', colour:Colour::Black};     p[7][1] = Piece{character:'h', colour:Colour::White};
        p[1][2] = Piece{character:'p', colour:Colour::Black};     p[7][2] = Piece{character:'b', colour:Colour::White};
        p[1][3] = Piece{character:'p', colour:Colour::Black};     p[7][3] = Piece{character:'q', colour:Colour::White};
        p[1][4] = Piece{character:'p', colour:Colour::Black};     p[7][4] = Piece{character:'k', colour:Colour::White};
        p[1][5] = Piece{character:'p', colour:Colour::Black};     p[7][5] = Piece{character:'b', colour:Colour::White};
        p[1][6] = Piece{character:'p', colour:Colour::Black};     p[7][6] = Piece{character:'h', colour:Colour::White};
        p[1][7] = Piece{character:'p', colour:Colour::Black};     p[7][7] = Piece{character:'c', colour:Colour::White};

        //return the board
        Board {
            pieces: p,
            white_score: 0,
            black_score: 0,
        }
    }

    pub fn print_board(&self) {
        let mut line: i8 = 0;
        println!();
        print!("0 ");
        for (_i, row) in self.pieces.iter().enumerate() {
            for (_y, col) in row.iter().enumerate() {
                print!("{} ", col.character);
            }
            println!();
            line = line+1;
            if line != 8 { print!("{} ",line);}
        }
        println!("  0 1 2 3 4 5 6 7");
        println!();
    }

    pub fn make_move(&mut self, piece_pos: &String, target_pos: &String, turn: Colour) -> bool {
        //convert to (x, y);
        //checks that coordinates are valid numbers and on the board
        let piece_pos = match convert(piece_pos.trim()) {
            Some(coords) => coords,
            None => {println!("Bad piece position"); return false}
        };
        let target_pos = match convert(target_pos.trim()) {
            Some(coords) => coords,
            None => {println!("Bad target position"); return false}
        };

        //check its a valid move
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

    pub fn valid_move(&mut self, piece_pos: &(usize, usize), target_pos: &(usize, usize), turn:&Colour) -> bool {

        let piece = self.pieces[piece_pos.1][piece_pos.0];

        //if valid piece, then get the possible moves for that piece
        if self.check_valid_piece(&piece_pos, &turn) {
            
            let moves = self.get_piece_moves(piece_pos);
            
            //  for a_move in moves.iter() {
            //     print!("{}, {}", a_move.0, a_move.1);

            //     if a_move == target_pos {
            //         print!(" <-- this one");
            //     } 
            //     println!();
            //  }

            if moves.contains(target_pos) {
                self.pieces[target_pos.1][target_pos.0].character = piece.character;
                self.pieces[target_pos.1][target_pos.0].colour = piece.colour;
                self.pieces[piece_pos.1][piece_pos.0].character = '.';
                self.pieces[piece_pos.1][piece_pos.0].colour = Colour::Empty;
            }
            
        }
        false
        //iterate through pieces and find correct position
    }

    pub fn check_valid_piece(&self, piece_pos:  &(usize, usize), turn:&Colour) -> bool {
        //y is x and x is y :((
        let piece = self.pieces[piece_pos.1][piece_pos.0];
        //check same colour
        if piece.colour == *turn {
            true
        } else if piece.colour == Colour::Empty {
            println!("Empty piece");
            return false;
        } else {
            println!("That is not your colour!");
            return false;
        }
    }

    //easier to implment this function now, rather than check that the single move is valid, as we can reuse in the AI
    pub fn get_piece_moves(&self, piece_pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let piece = self.pieces[piece_pos.1][piece_pos.0];
        //pattern match to each piece
        match piece.character {
            //each piece
            'p' => {self.pawn_moves(piece_pos)},
            'c' => {self.castle_moves(piece_pos)},
            'h' => {self.knight_moves(piece_pos)},
            'b' => {self.bishop_moves(piece_pos)},
            'k' => {self.king_moves(piece_pos)},
            'q' => {self.queen_moves(piece_pos)},
            _ => vec![]
        }
    }

    //refactor these into separate files?
    //returns list of coords of possible pawn moves
    pub fn pawn_moves(&self, piece_pos: &(usize, usize)) -> Vec<(usize, usize)> {
        
        let piece = self.pieces[piece_pos.1][piece_pos.0];
        //vector of unknown size to which we will push the moves
        //convert into slice then return
        let mut moves: Vec<(usize, usize)> = Vec::new();

        match piece.colour {    //fuckin love this match thing
            //white moves up
            Colour::White => {
                //can move two pieces forward IF in one of the original starting position
                if piece_pos.1 == 6 {
                    //can move two pieces forward (if not another piece there)
                    if self.pieces[piece_pos.1-2][piece_pos.0].colour == Colour::Empty {
                        moves.push((piece_pos.0, piece_pos.1 - 2));
                    }
                }
                //as long as piece isnt at end of board
                if piece_pos.1 != 0 {
                    //piece can move forward one
                    if self.pieces[piece_pos.1-1][piece_pos.0].colour == Colour::Empty {
                        moves.push((piece_pos.0, piece_pos.1 - 1));
                    }
                    //or up right or up left if there is a black piece there
                    //uh oh need to do check to see if left is within the board
                    if piece_pos.0 != 0 {
                        println!("{}", piece_pos.0);
                        if self.pieces[piece_pos.1 - 1][piece_pos.0 - 1].colour == Colour::Black {
                            moves.push((piece_pos.0 - 1, piece_pos.1 - 1));
                        }
                    }
                    if piece_pos.0 + 1 < 8 {
                        if self.pieces[piece_pos.1 - 1][piece_pos.0 + 1].colour == Colour::Black {
                            moves.push((piece_pos.0 + 1, piece_pos.1 - 1));
                        }
                    }
                }
            }
            //black moves down
            Colour::Black => {
                //can move two pieces forward IF in one of the original starting position
                if piece_pos.1 == 1 {
                    //can move two pieces forward (if not another piece there)
                    if self.pieces[piece_pos.1+2][piece_pos.0].colour == Colour::Empty {
                        moves.push((piece_pos.0, piece_pos.1 + 2));
                    }
                }
                //as long as piece isnt at end of board
                if piece_pos.1 != 7 {
                    //piece can move forward one
                    if self.pieces[piece_pos.1+1][piece_pos.0].colour == Colour::Empty {
                        moves.push((piece_pos.0, piece_pos.1 + 1));
                    }
                    //or up right or up left if there is a white piece there
                    if piece_pos.0 != 0 {
                        if self.pieces[piece_pos.1 + 1][piece_pos.0 + 1].colour == Colour::White {
                            moves.push((piece_pos.0 + 1, piece_pos.1 + 1));
                        }
                    }
                    if piece_pos.0 + 1 < 8 {
                        if self.pieces[piece_pos.1 + 1][piece_pos.0 - 1].colour == Colour::White {
                            moves.push((piece_pos.0 - 1, piece_pos.1 + 1));
                        }
                    }
                }
            },
            //else ?
            _  => {println!("why are we here just to suffer")}
        }

        moves
    }

    //returns list of coords of possible castle moves
    pub fn castle_moves(&self, piece_pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let piece = self.pieces[piece_pos.1][piece_pos.0];
        //vector of unknown size to which we will push the moves
        //convert into slice then return
        let mut moves: Vec<(usize, usize)> = Vec::new();

        //iterate up
        for i in (0..piece_pos.1).rev() {
            //check if this piece is a same colour or opposite
            //if oppose, include this move in list of moves
            if self.pieces[i][piece_pos.0].colour != Colour::Empty {
                //if same colour, simply break from loop
                if self.pieces[i][piece_pos.0].colour == piece.colour {
                    break;
                } else {
                    moves.push((piece_pos.0, i));
                    break;
                }
            } else {
                //if empty we just add the move to the list
                moves.push((piece_pos.0, i));
            }
        }
        //iterate down
        for i in (piece_pos.1+1)..8 {
            if self.pieces[i][piece_pos.0].colour != Colour::Empty {
                //if same colour, simply break from loop
                if self.pieces[i][piece_pos.0].colour == piece.colour {
                    break;
                } else {
                    moves.push((piece_pos.0, i));
                    break;
                }
            } else {
                //if empty we just add the move to the list
                moves.push((piece_pos.0, i));
            }
        }
        //iterate right
        for i in (piece_pos.0+1)..8 {
            if self.pieces[piece_pos.1][i].colour != Colour::Empty {
                //if same colour simply break from loop
                if self.pieces[piece_pos.1][i].colour == piece.colour {
                    break;
                } else {
                    moves.push((i, piece_pos.1));
                    break;
                }
            } else {
                //empty piece so can add move
                moves.push((i, piece_pos.1));
            }
        }
        //iterate left
        for i in (0..piece_pos.0).rev() {
            if self.pieces[piece_pos.1][i].colour != Colour::Empty {
                //if same colour simply break from loop
                if self.pieces[piece_pos.1][i].colour == piece.colour {
                    break;
                } else {
                    moves.push((i, piece_pos.1));
                    break;
                }
            } else {
                //empty piece so can add move
                moves.push((i, piece_pos.1));
            }
        }

        moves
    }

    //returns list of coords of possible knight moves
    pub fn knight_moves(&self, piece_pos: &(usize, usize)) -> Vec<(usize, usize)> {
        //TODO Implement this shit

        let piece = self.pieces[piece_pos.1][piece_pos.0];
        //vector of unknown size to which we will push the moves
        //convert into slice then return
        let mut moves: Vec<(usize, usize)> = Vec::new();

        vec![]
    }

    //returns list of coords of possible bishop moves
    fn bishop_moves(&self, piece_pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let piece = self.pieces[piece_pos.1][piece_pos.0];
        //vector of unknown size to which we will push the moves
        //convert into slice then return
        let mut moves: Vec<(usize, usize)> = Vec::new();

        //iterate right
        let mut j = 0;  //j is the y down direction ?
        let mut j_up = 8;   //j_up is the y up direction ?  //so we dont do too many for loops lol
        for i in (piece_pos.0+1)..8 {
            j = j + 1;    //iterate down one
            
            if j < 8 {  //prevents out of bounds exception
                if self.pieces[j][i].colour != Colour::Empty {
                    //if same colour simply break from loop
                    if self.pieces[j][i].colour == piece.colour {
                        break;
                    } else {
                        moves.push((i, j));
                        break;
                    }
                } else {
                    //empty piece so can add move
                    moves.push((i, j));
                }
            } 

            //now check the up direction
            j_up = j_up - 1;    //iterate up one
            if j_up >= 0 {
                if self.pieces[j_up][i].colour != Colour::Empty {
                    //if same colour simply break from loop
                    if self.pieces[j_up][i].colour == piece.colour {
                        break;
                    } else {
                        moves.push((i, j_up));
                        break;
                    }
                } else {
                    //empty piece so can add move
                    moves.push((i, j_up));
                }
            }
        }

        //iterate left
        j = 0;
        j_up = 0;
        for i in (0..piece_pos.0).rev() {
            if j < 8 {  //prevents out of bounds exception
                if self.pieces[j][i].colour != Colour::Empty {
                    //if same colour simply break from loop
                    if self.pieces[j][i].colour == piece.colour {
                        break;
                    } else {
                        moves.push((i, j));
                        break;
                    }
                } else {
                    //empty piece so can add move
                    moves.push((i, j));
                }
            } 

            //now check the up direction
            j_up = j_up - 1;    //iterate up one
            if j_up >= 0 {
                if self.pieces[j_up][i].colour != Colour::Empty {
                    //if same colour simply break from loop
                    if self.pieces[j_up][i].colour == piece.colour {
                        break;
                    } else {
                        moves.push((i, j_up));
                        break;
                    }
                } else {
                    //empty piece so can add move
                    moves.push((i, j_up));
                }
            }
        }
        vec![]
    }
    
    //returns list of coords of possible king moves
    fn king_moves(&self, piece_pos: &(usize, usize)) -> Vec<(usize, usize)> {

        let piece = self.pieces[piece_pos.1][piece_pos.0];
        //vector of unknown size to which we will push the moves
        //convert into slice then return
        let mut moves: Vec<(usize, usize)> = Vec::new();

        //up one
        if piece_pos.1 != 0 {
            if self.pieces[piece_pos.1 - 1][piece_pos.0].colour != Colour::Empty {
                //if not same colour can take piece
                // TODO: check that position being moved to isn't under check 
                if self.pieces[piece_pos.1 - 1][piece_pos.0].colour != piece.colour {
                    moves;
                }
            } else {
                moves.push((piece_pos.1 - 1, piece_pos.0));
            }
        }
        vec![]
    }

    //returns list of coords of possible queen moves
    fn queen_moves(&self, piece_pos: &(usize, usize)) -> Vec<(usize, usize)> {
        vec![]
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
