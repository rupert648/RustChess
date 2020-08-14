use crate::board;
use crate::board::convert;
use crate::ai;
use std::io;

/// Information about the game is held here
struct Game {
    turn: board::Colour,
    board: board::Board,
    ai: ai::AI,
}

impl Game {
    // function returns an instance of the game struct with given values
    fn init_game(turn: board::Colour, ai: ai::AI) -> Game {
        Game {
            turn,
            board: board::Board::new(),
            ai
        }
    }
}

//function to run the game
pub fn run_game() {

    let mut good_res = false;   // good response?
    let mut ai = ai::AI::new(board::Colour::Empty, 0);  //initiate the AI

    //loop until a good response is seen
    while !good_res {

        /*
            This code handles the player requests as to whether or not they want an AI
            Might refactor to be included in the program arguments
        */
        println!("Do you want an AI? (y/n)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read line");

        //compare response
        match &response.to_lowercase().trim()[..] {
            "y" => {
                println!("Which colour would you like the AI to play?");
                println!("1) White\n2) Black");
                let mut response2 = String::new();
                io::stdin().read_line(&mut response2).expect("Failed to read line");
                match &response2.trim()[..] {
                    "1" => {ai.set_colour(board::Colour::White); good_res = true},
                    "2" => {ai.set_colour(board::Colour::Black); good_res = true},
                    _ => println!("that aint 1 or two"),
                }
            },
            "n" => {
                good_res = true
            }
            _ => {}
        }
    }

    //initiate game
    let mut game = Game::init_game(board::Colour::White, ai);
    
    //loop the game
    loop {
        //print Board
        game.board.print_board();
        //Print players move
        
        /*
            This block of code loops until it has both the starting and target position from the player
            This code will also get the AI's move if there is an AI.
        */
        let mut valid_move = false;
        while !valid_move {

            let piece_to_move;
            let target_position;

            // if it is the AI's turn
            if game.turn == game.ai.colour {
                println!("it is the ai's turn");

                // get the AI's move
                let result = game.ai.get_move(&game.board);

                // separate out tuple
                piece_to_move = result.0;
                target_position = result.1;

            } else {

                //get the player's move
                println!("It is {:?} Turn", game.turn);
                println!("Piece to move:"); 
                piece_to_move = match get_position() {
                    Some(x) => match convert(&x.trim()) {
                        Some(coords) => coords,
                        None => {println!("Bad piece position"); continue}
                    },
                    None => return,
                };
                println!("Target position:  ");
                target_position = match get_position() {
                    Some(x) => match convert(&x.trim()) {
                        Some(coords) => coords,
                        None => {println!("Bad piece position"); continue}
                    },
                    None => return,
                }
            }
            //formatting
            println!();

            //attempt to make a move
            if game.board.make_move(piece_to_move, target_position, game.turn) {
                valid_move = true;
            } else {
                println!("Bad move, try again");
            }

        }
        
        //switch to other players turn
        game.turn = if game.turn == board::Colour::White { board::Colour::Black } else { board::Colour::White };
    }
}

pub fn get_position() -> Option<String> {

    //get piece player wants to move
    let mut piece_to_move = String::new();
    io::stdin().read_line(&mut piece_to_move).expect("Failed to read line");

    if piece_to_move.trim() == "quit" {
        return None;
    }
    Some(piece_to_move)
}