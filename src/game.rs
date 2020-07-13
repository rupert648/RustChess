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

            //get piece player wants to move
            println!("Piece to move:"); 
            let mut piece_to_move = String::new();
            io::stdin().read_line(&mut piece_to_move).expect("Failed to read line");
            
            //check if they've quit
            if piece_to_move.trim() == "quit" {
                println!("thanks for playing!");
                std::process::exit(1);
            }

            //get target position
            println!("Target position:  ");
            let mut target_position = String::new();
            io::stdin().read_line(&mut target_position).expect("Failed to read line");  

            //formatting
            println!();

            //check again if theyve quit
            if target_position.trim() == "quit" {
                println!("thanks for playing!");
                std::process::exit(1);
            }


            //attempt to make a move
            if game.board.make_move(&piece_to_move, &target_position, game.turn) {
                valid_move = true;
            } else {
                println!("Bad move, try again");
            }

        }
        //check win status

        //switch to other players turn
        game.turn = if game.turn == board::Colour::White { board::Colour::Black } else { board::Colour::White };
    }
}