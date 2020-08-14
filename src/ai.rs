extern crate rand; // 0.6.5

use rand::Rng;
use crate::board::Colour;
use crate::board::Board;

pub struct AI {
    pub colour: Colour,
    level: i8,
}

impl AI {
    pub fn new(colour: Colour, level: i8) -> AI {
        AI {
            colour,
            level
        }
    }

    pub fn set_colour(&mut self, new: Colour) {
        self.colour = new;
    }

    pub fn get_move(&self, board: &Board) -> ((usize, usize), (usize, usize)) {
        //start with a random move

        let moves = self.get_all_moves(board);

        //select a random number
        let num = rand::thread_rng().gen_range(0, moves.len());
        println!("{}", num);
        let random_result = match moves.get(num) {
            Some(x) => x,
            None => &((0,0), (0,0))
        };

        //take and return that move
        *random_result
    }

    //returns array of starting and finishing positions
    fn get_all_moves(&self, board: &Board) -> Vec<((usize, usize), (usize, usize))> {

        //iterate through all the pieces and get the coordinates of the pieces of the ai colour
        //TODO: refactor to skip this step  -- pieces attribute in ai
        let mut pieces: Vec<(usize, usize)> = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for i in board.pieces.iter() {  
            for j in i.iter() {
                //if same colour, add to vector
                if j.colour == self.colour {
                    pieces.push((x, y));
                }

                x = x + 1;
            }
            y = y + 1;
            x = 0;
        }  
        
        let mut moves: Vec<((usize, usize), (usize, usize))> = Vec::new();
        //now we have list of pieces.
        for p in pieces.iter() {
            //list of possible moves for that piece
            let this_moves = board.get_piece_moves(p);
            //for each of these moves push the original piece position and each of its moves
            for m in this_moves.iter() {
                moves.push((*p, *m));
            }
        }


        moves
    }
}