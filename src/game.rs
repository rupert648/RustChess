pub mod board;
use std::io;

struct Game {
    turn: board::Colour,
    board: board::Board,
}

impl Game {
    fn init_game(turn: board::Colour) -> Game {
        Game {
            turn,
            board: board::Board::new(),
        }
    }
}

pub fn run_game() {

    let mut game = Game::init_game(board::Colour::White);
    
    loop {
        //print Board
        game.board.print_board();
        //Print players move
        match game.turn {
            board::Colour::White => println!("It is Whites Turn"),
            board::Colour::Black => println!("It is Blacks Turn"),
            _ => println!("ahhhh"), //should never happen but need to be exhaustive
        };
        println!("Move should be in format x,y");
        //get players move
        let mut valid_move = false;
        while !valid_move {

            println!("Piece to move:"); 
            let mut piece_to_move = String::new();
            //read into piece_to_move
            io::stdin()
                .read_line(&mut piece_to_move)
                .expect("Failed to read line");
            
            if piece_to_move.trim() == "quit" {
                println!("thanks for playing!");
                std::process::exit(1);
            }
            println!("Target position:  ");
            let mut target_position = String::new();
            io::stdin()
                .read_line(&mut target_position)
                .expect("Failed to read line");  
            println!();
            if game.board.make_move(&piece_to_move, &target_position, game.turn) {
                valid_move = true;
            } else {
                println!("Bad move, try again");
            }

        }
        

        //check valid move
        //update board
        //check win status
        //change turn
    }
}