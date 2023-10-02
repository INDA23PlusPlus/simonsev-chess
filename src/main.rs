mod lib;
use lib::*;
fn main() {
    let mut game = Game::new();


    game.print_board();
    let from = String::from("E2");
    let to = String::from("E3");
    game.input_move(from , to);
    game = game.do_turn();

    /*
    game.print_board();
    let from = String::from("F7");
    let to = String::from("F6");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("E3");
    let to = String::from("E4");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("E8");
    let to = String::from("F7");
    game.input_move(from , to);
    game = game.do_turn();
    */

    game.print_board();
    let from = String::from("A7");
    let to = String::from("A5");
    game.input_move(from , to);
    game = game.do_turn();

    
    game.print_board();
    let from = String::from("D1");
    let to = String::from("H5");
    game.input_move(from , to);
    game = game.do_turn(); 

    game.print_board();
    let from = String::from("A8");
    let to = String::from("A6");
    game.input_move(from , to);
    game = game.do_turn(); 

    game.print_board();
    let from = String::from("H5");
    let to = String::from("A5");
    game.input_move(from , to);
    game = game.do_turn(); 
    
    game.print_board();
    let from = String::from("H7");
    let to = String::from("H5");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("H2");
    let to = String::from("H4");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("A6");
    let to = String::from("H6");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("A5");
    let to = String::from("C7");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("F7");
    let to = String::from("F6");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("C7");
    let to = String::from("D7");
    game.input_move(from , to);
    game = game.do_turn();
    game.print_board();

    let from = String::from("E8");
    let to = String::from("F7");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    
    let from = String::from("H7");
    let to = String::from("H5");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("D7");
    let to = String::from("B7");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("D8");
    let to = String::from("D3");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("B7");
    let to = String::from("B8");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("D3");
    let to = String::from("H7");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("B8");
    let to = String::from("C8");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("F7");
    let to = String::from("G6");
    game.input_move(from , to);
    game = game.do_turn();

    game.print_board();
    let from = String::from("C8");
    let to = String::from("E6");
    game.input_move(from , to);
    game = game.do_turn();

    //game.print_board();
    //game.print_check_board(true);
    /*
    game.print_board();
    let from = String::from("H7");
    let to = String::from("H5");
    game.input_move(from , to);
    game = game.do_turn();*/

    //game.print_board();
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