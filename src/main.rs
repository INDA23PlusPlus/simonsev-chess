mod lib;
use lib::*;
fn main() {
    let mut game = Game::new();

    game.print_board();
    let from = String::from("E2");
    let to = String::from("E4");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("E7");
    let to = String::from("E5");
    game.input_move(from , to);
    game = game.do_turn(); 
    
    game.print_board();
    let from = String::from("E4");
    let to = String::from("E5");
    game.input_move(from , to);
    game = game.do_turn(); 

    game.print_board();
    let from = String::from("A2");
    let to = String::from("A3");
    game.input_move(from , to);
    game = game.do_turn(); 

    game.print_board();
    let from = String::from("F7");
    let to = String::from("F5");
    game.input_move(from , to);
    game = game.do_turn(); 
    
    game.print_board();
    let from = String::from("E4");
    let to = String::from("F5");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    /*
    let from = String::from("E2");
    let to = String::from("E4");
    game.input_move(from , to);
    game.do_turn(); 
    
    game.print_board();
    let from = String::from("E2");
    let to = String::from("E4");
    game.input_move(from , to);
    game.do_turn(); 
    
    game.print_board();
    let from = String::from("E2");
    let to = String::from("E4");
    game.input_move(from , to);
    game.do_turn();*/


}