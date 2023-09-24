mod utils;
use utils::*;

fn main() {

    let mut game = Game::new();
    let from = String::from("E2");
    let to = String::from("E3");
    game.input_move(from, to);
    game = game.do_turn();
    game.print_board();

    let from = String::from("A7");
    let to = String::from("A6");
    game.input_move(from, to);
    game = game.do_turn();
    game.print_board();
    
    let from = String::from("D1");
    let to = String::from("H5");
    game.input_move(from, to);
    game = game.do_turn();
    game.print_board();

    let from = String::from("A6");
    let to = String::from("A5");
    game.input_move(from, to);
    game = game.do_turn();
    game.print_board();

    let from = String::from("F1");
    let to = String::from("C4");
    game.input_move(from, to);
    game = game.do_turn();
    game.print_board();

    let from = String::from("F7");
    let to = String::from("F6");
    game.input_move(from, to);
    game = game.do_turn();
    game.print_board();

    let from = String::from("H5");
    let to = String::from("F7");
    game.input_move(from, to);
    game = game.do_turn();
    game.print_board();
}