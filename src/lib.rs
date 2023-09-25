use std::clone::Clone;
use std::cmp::PartialEq;
use std::io;

#[derive(Clone)]

//
// move_history not yet implemented
pub struct Game {
    boards: Boards,
    pub white_turn: bool,
    pub move_history: Vec<String>,
    w_king_pos: Move,
    b_king_pos: Move,
    move_from: String,
    move_to: String,
    pub mate: bool,
}

impl Game {

    // Returns an instance of game, ready to be played :)
    pub fn new() -> Game {
        let mut game = Game {
            boards: Boards {
                board: build_board(),
                white_check_board: build_check_board(),
                black_check_board: build_check_board(),
            },
            white_turn: true,
            move_history: Vec::new(),
            w_king_pos: Move { x: 0, y: 4 },
            b_king_pos: Move { x: 7, y: 4 },
            move_from: String::new(),
            move_to: String::new(),
            mate: false,
        };
        game = game.find_all_moves();
        game = game.clear_self_checking_moves();
        game
    }

    // Takes one String as input, checks if it is formatted 
    // correctly to use as a move, returns false if not and vice versa
    fn check_input(input: &String) -> bool {

        if input.len() != 2 {
            return false;
        }

        let x = input.chars().nth(1).unwrap() as u8;
        let y = input.chars().nth(0).unwrap() as u8;
        if x < 49 || x > 56 || y < 65 || y > 72 {
            return false;
        }
        true
    }

    // Takes two strings as input 
    // Sets move_from (in self) to first input and move_to (in self) to second input
    // Input should be formatted as board positions 
    // i.e. first a capital letter from A to H (incluive)
    // and then a number from 1 to 8 (inclusic)
    // and of length 2
    pub fn input_move(&mut self, move_from: String, move_to: String) {
        self.move_from = move_from;
        self.move_to = move_to;
    }
    
    // Does a turn, updates the board and checks for mate, returns self
    // If the game is mate, whose turn it is will not change
    // Returns self early without doing anything if move_from and move_to 
    // are improperly formatted or if the move is not valid
    pub fn do_turn(mut self) -> Game{
        if !Self::check_input(&self.move_from) || !Self::check_input(&self.move_from) {
            self.reset_moves();
            return self;
        }

        if !&self.check_move_valid(){
            self.reset_moves();
            return self;
        }

        self = self.do_move();

        self = self.find_all_moves();
        self = self.clear_self_checking_moves();
        self.reset_moves();

        if(self.check_for_mate()) {
            self.mate = true;
            return self;
        }

        self.white_turn = !self.white_turn;
        self
    }
    
    fn reset_moves(&mut self){
        self.move_from = String::from("");
        self.move_to = String::from("2");
    }

    // Returns clone of board
    pub fn get_board(&self) -> Vec<Vec<Square>> {
        return self.boards.board.clone();
    }

    fn check_move_valid(&self) -> bool {
        let mv_from = string_to_move(&self.move_from);
        let mv_to = string_to_move(&self.move_to);

        if !self.boards.board[mv_from.x as usize][mv_from.y as usize].occupied {
            return false;
        }
        if self.boards.board[mv_from.x as usize][mv_from.y as usize].piece.white != self.white_turn {
            return false;
        }
        if !self.boards.board[mv_from.x as usize][mv_from.y as usize].piece.moves.contains(&mv_to){
            return false;
        }
        true
    }

    pub fn run_game(mut self) -> Game {
        self = self.find_all_moves();
        self = self.clear_self_checking_moves();
        loop {
            self.print_board();
            self = self.find_all_moves();
            self = self.take_turn();

            if (self.check_for_mate()) {
                return self;
            }

            self.white_turn = !self.white_turn;
        }
    }

    pub fn take_turn(mut self) -> Game {
        let mut valid_move = false;

        if self.white_turn {
            println!("White's turn!");
        } else {
            println!("Black's turn!");
        }

        self = self.find_all_moves();
        self = self.clear_self_checking_moves();

        while !valid_move {
            let from = Game::take_input();
            let mv_from = string_to_move(&from);
            if self.boards.board[mv_from.x as usize][mv_from.y as usize].occupied {
                if self.boards.board[mv_from.x as usize][mv_from.y as usize]
                    .piece
                    .white
                    == self.white_turn
                {
                    let to = Game::take_input();
                    let mv_to = string_to_move(&to);
                    let mv_vec = &self.boards.board[mv_from.x as usize][mv_from.y as usize]
                        .piece
                        .moves;

                    if self.boards.board[mv_from.x as usize][mv_from.y as usize]
                        .piece
                        .moves
                        .contains(&mv_to)
                    {
                        self = self.do_move();
                        valid_move = true;
                    } else {
                        println!("This is not a valid move!");
                    }
                } else {
                    println!("This is not your piece!");
                }
            } else {
                println!("This square does not have a piece!");
            }
        }
        self.print_board();
        self = self.find_all_moves();
        self = self.clear_self_checking_moves();

        self
    }

    pub fn check_for_mate(&self) -> bool {
        let mut mate = true;
        if self.white_turn {
            if self.boards.white_check_board[self.b_king_pos.x as usize][self.b_king_pos.y as usize] {
                for i in 0..8 {
                    for j in 0..8 {
                        if self.boards.board[i][j].occupied {
                            if !self.boards.board[i][j].piece.white {
                                if self.boards.board[i][j].piece.moves.len() != 0 {
                                    mate = false;
                                }else {
                                    
                                }
                            }
                        }
                    }
                }
            } else {
                mate = false;
            }
        } else {
            if self.boards.black_check_board[self.w_king_pos.x as usize][self.w_king_pos.y as usize] {
                for i in 0..8 {
                    for j in 0..8 {
                        if self.boards.board[i][j].occupied {
                            if self.boards.board[i][j].piece.white {
                                if self.boards.board[i][j].piece.moves.len() != 0 {
                                    mate = false;
                                }
                            }
                        }
                    }
                }
            } else {
                mate = false;
            }
        }
        if mate && self.white_turn {
            println!("Black is mate!");
        } else if mate {
            println!("White is mate!");
        }
        return mate;
    }

    pub fn take_input() -> String {
        let mut user_in = String::new();
        let stdin = io::stdin();
        let _ = io::stdin().read_line(&mut user_in);
        println!("You chose: {}", user_in);
        user_in
    }

    pub fn clear_self_checking_moves(mut self) -> Game {
        let mut boards_ = self.boards.clone();

        let mut w_king = Move { x: 0, y: 0 };
        let mut b_king = Move { x: 0, y: 0 };

        for i in (0..8) {
            for j in (0..8) {
                match boards_.board[i][j].piece.piece_type {
                    PieceType::King => {
                        if boards_.board[i][j].piece.white {
                            w_king = Move {
                                x: i as u8,
                                y: j as u8,
                            }
                        } else {
                            b_king = Move {
                                x: i as u8,
                                y: j as u8,
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        let boards_clone = boards_.clone();
        for i in (0..8) {
            for j in (0..8) {
                if boards_.board[i][j].occupied {
                    boards_ = self.boards.clone();
                    let mut indexes_to_pop: Vec<usize> = Vec::new();

                    let white_ = boards_.board[i][j].piece.white;
                    for k in 0..boards_.board[i][j].piece.moves.len() {
                        boards_ = boards_clone.clone();
                        boards_.board = move_piece(
                            &Move {
                                x: i as u8,
                                y: j as u8,
                            }
                            .move_to_string(),
                            &boards_.board[i][j].piece.moves[k].move_to_string(),
                            boards_.board,
                        );

                        boards_ = find_all_moves(
                            boards_.board,
                            boards_.white_check_board,
                            boards_.black_check_board,
                        );

                        if white_ {
                            if boards_.black_check_board[w_king.x as usize][w_king.y as usize] {
                                indexes_to_pop.push(k);
                            }
                        } else {
                            if boards_.white_check_board[b_king.x as usize][b_king.y as usize] {
                                indexes_to_pop.push(k);
                            }
                        }
                    }
                    for h in 0..indexes_to_pop.len() {
                        for l in &self.boards.board[i][j].piece.moves {
                        }
                        self.boards.board[i][j]
                            .piece
                            .moves
                            .remove(indexes_to_pop[h] - h);
                    }
                }
            }
        }
        self
    }

    pub fn do_move(mut self) -> Game {
        let mut from = string_to_move(&self.move_from);
        let mut to = string_to_move(&self.move_to);

        if (from == self.w_king_pos) {
            self.w_king_pos = to.clone();
        } else if (from == self.w_king_pos) {
            self.b_king_pos = to.clone();
        }

    
        square_to_square(&from, to, &mut self.boards.board);
        square_to_unoccupied(&from, &mut self.boards.board);


        self
    }

    pub fn find_all_moves(mut self) -> Game {
        let mut boards_ = self.boards;
        self.boards = find_all_moves(
            boards_.board,
            boards_.white_check_board,
            boards_.black_check_board,
        );
        self
    }

    pub fn print_board(&self) {
        let board = &self.boards.board;

        for i in (0..board.len()).rev() {
            print!("{} ", i + 1);
            for j in (0..board[i].len()) {
                board[i][j].print_square();
            }
            println!();
        }
        print!("    ");
        for i in (1..9) {
            print!("{:<5}", ((i + 64) as u8) as char);
        }
        println!("");
    }

    pub fn print_check_board(&self, white: bool) {
        let mut check_board;
        if white {
            check_board = &self.boards.white_check_board;
        } else {
            check_board = &self.boards.black_check_board;
        }

        for i in (0..check_board.len()).rev() {
            print!("{:?} ", i + 1);
            for j in (0..check_board[i].len()) {
                print!("[{:<5}]", check_board[i][j]);
            }
            println!("");
        }
        print!("    ");
        for i in (0..check_board[0].len()) {
            print!("{:<7}", ((i + 65) as u8) as char);
        }
        println!("");
    }
}

#[derive(Clone)]
pub struct Square {
    pub x: u8,
    pub y: u8,
    pub piece: Piece,
    pub occupied: bool,
}

impl Square {
    pub fn print_square(&self) {
        print!("[");
        match &self.piece.piece_type {
            PieceType::Pawn => print!("P "),
            PieceType::Rook => print!("R "),
            PieceType::Knight => print!("Kn"),
            PieceType::Bishop => print!("B "),
            PieceType::Queen => print!("Q "),
            PieceType::King => print!("K "),
            PieceType::Unoccupied => {
                print!(".  ]");
                return;
            }
        }
        match &self.piece.white {
            true => print!("W"),
            false => print!("B"),
        }
        print!("]");
    }
}

#[derive(Clone)]
pub struct Boards {
    pub board: Vec<Vec<Square>>,
    pub white_check_board: Vec<Vec<bool>>,
    pub black_check_board: Vec<Vec<bool>>,
}

#[derive(Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub white: bool,
    pub moves: Vec<Move>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Unoccupied,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Move {
    pub x: u8,
    pub y: u8,
}

fn build_check_board() -> Vec<Vec<bool>> {
    let mut check_board: Vec<Vec<bool>> = Vec::new();
    for i in 0..8 {
        check_board.push(Vec::new());
        for j in 0..8 {
            check_board[i].push(false);
        }
    }
    check_board
}

pub fn reset_check_board(mut check_board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for i in 0..check_board.len() {
        for j in 0..check_board[i].len() {
            check_board[i][j] = false;
        }
    }
    check_board
}

pub fn find_all_moves(
    mut board: Vec<Vec<Square>>,
    mut white_check_board: Vec<Vec<bool>>,
    mut black_check_board: Vec<Vec<bool>>,
) -> Boards {
    white_check_board = reset_check_board(white_check_board.to_owned());
    black_check_board = reset_check_board(black_check_board.to_owned());
    let mut king1 = Move { x: 0, y: 0 };
    let mut king2 = Move { x: 0, y: 0 };
    let mut king_found = false;

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j].occupied {
                if board[i][j].piece.piece_type == PieceType::King {
                    if !king_found {
                        king1 = Move {
                            x: i as u8,
                            y: j as u8,
                        };
                        king_found = true;
                    } else {
                        king2 = Move {
                            x: i as u8,
                            y: j as u8,
                        };
                    }
                } else {
                    generate_moves(
                        i,
                        j,
                        &mut board,
                        &mut white_check_board,
                        &mut black_check_board,
                    );
                }
            }
        }
    }
    generate_moves(
        king1.x as usize,
        king1.y as usize,
        &mut board,
        &mut white_check_board,
        &mut black_check_board,
    );
    generate_moves(
        king2.x as usize,
        king2.y as usize,
        &mut board,
        &mut white_check_board,
        &mut black_check_board,
    );

    let mut boards = Boards {
        board: board,
        white_check_board: white_check_board,
        black_check_board: black_check_board,
    };
    boards
}

pub fn move_piece(from: &String, to: &String, mut board: Vec<Vec<Square>>) -> Vec<Vec<Square>> {
    let mut move_from = string_to_move(&from);
    let mut move_to = string_to_move(&to);

    if board[move_from.x as usize][move_from.y as usize]
        .piece
        .moves
        .contains(&move_to)
    {
        square_to_square(&move_from, move_to, &mut board);
        square_to_unoccupied(&move_from, &mut board);
    } else {
        println!("Move not found");
    }

    board
}

pub fn square_to_unoccupied(pos: &Move, board: &mut Vec<Vec<Square>>) {
    board[pos.x as usize][pos.y as usize].piece.piece_type = PieceType::Unoccupied;
    board[pos.x as usize][pos.y as usize].piece.white = true;
    board[pos.x as usize][pos.y as usize].piece.moves = Vec::new();
    board[pos.x as usize][pos.y as usize].occupied = false;
}

pub fn generate_moves(
    x: usize,
    y: usize,
    board: &mut Vec<Vec<Square>>,
    mut white_check_board: &mut Vec<Vec<bool>>,
    mut black_check_board: &mut Vec<Vec<bool>>,
) {
    match board[x as usize][y as usize].piece.piece_type {
        PieceType::Pawn => {
            board[x as usize][y as usize].piece.moves = moves_pawn(
                &board[x as usize][y as usize],
                board,
                &mut white_check_board,
                &mut black_check_board,
            )
        }
        PieceType::Rook => {
            board[x as usize][y as usize].piece.moves = moves_rook(
                &board[x as usize][y as usize],
                board,
                &mut white_check_board,
                &mut black_check_board,
            )
        }
        PieceType::Knight => {
            board[x as usize][y as usize].piece.moves = moves_knight(
                &board[x as usize][y as usize],
                board,
                &mut white_check_board,
                &mut black_check_board,
            )
        }
        PieceType::Bishop => {
            board[x as usize][y as usize].piece.moves = moves_bishop(
                &board[x as usize][y as usize],
                board,
                &mut white_check_board,
                &mut black_check_board,
            )
        }
        PieceType::Queen => {
            board[x as usize][y as usize].piece.moves = moves_queen(
                &board[x as usize][y as usize],
                board,
                &mut white_check_board,
                &mut black_check_board,
            )
        }
        PieceType::King => {
            board[x as usize][y as usize].piece.moves = moves_king(
                &board[x as usize][y as usize],
                board,
                &mut white_check_board,
                &mut black_check_board,
            )
        }
        _ => (),
    }
}

pub fn print_check_board(mut check_board: &Vec<Vec<bool>>) {
    for i in (0..check_board.len()).rev() {
        for j in 0..check_board[i].len() {
            print!("{:<7}", check_board[i][j]);
        }
        println!("");
    }
}

pub fn square_to_square(move_from: &Move, move_to: Move, board: &mut Vec<Vec<Square>>) {
    match board[move_from.x as usize][move_from.y as usize]
        .piece
        .piece_type
    {
        PieceType::Pawn => {
            board[move_to.x as usize][move_to.y as usize]
                .piece
                .piece_type = PieceType::Pawn
        }
        PieceType::Rook => {
            board[move_to.x as usize][move_to.y as usize]
                .piece
                .piece_type = PieceType::Rook
        }
        PieceType::Knight => {
            board[move_to.x as usize][move_to.y as usize]
                .piece
                .piece_type = PieceType::Knight
        }
        PieceType::Bishop => {
            board[move_to.x as usize][move_to.y as usize]
                .piece
                .piece_type = PieceType::Bishop
        }
        PieceType::Queen => {
            board[move_to.x as usize][move_to.y as usize]
                .piece
                .piece_type = PieceType::Queen
        }
        PieceType::King => {
            board[move_to.x as usize][move_to.y as usize]
                .piece
                .piece_type = PieceType::King
        }
        PieceType::Unoccupied => {
            board[move_to.x as usize][move_to.y as usize]
                .piece
                .piece_type = PieceType::Unoccupied
        }
    }
    board[move_to.x as usize][move_to.y as usize].piece.white = board[move_from.x as usize]
        [move_from.y as usize]
        .piece
        .white;
    board[move_to.x as usize][move_to.y as usize].occupied =
        board[move_from.x as usize][move_from.y as usize].occupied;
}

pub fn print_board(mut board: &Vec<Vec<Square>>) {
    for i in (0..board.len()).rev() {
        for j in 0..board[i].len() {
            board[i][j].print_square();
        }
        println!("");
    }
}

impl Move {
    pub fn move_to_string(&self) -> String {
        let mut s = String::new();
        s.push((&self.y + 65) as char);
        s.push((&self.x + 49) as char);

        s
    }

    pub fn print_move(&self) {
        print!("{:?},", &self.x);
        println!("{:?}", &self.y);
    }
}

pub fn string_to_move(input: &String) -> Move {
    let mut x: &char = &input.chars().nth(1).unwrap();
    let mut y: &char = &input.chars().nth(0).unwrap();

    let mut x = x.to_owned() as u8;
    let mut y = y.to_owned() as u8;

    x -= 49;
    y -= 65;

    Move { x: x, y: y }
}

fn check_move(
    square: &Square,
    x: i8,
    y: i8,
    board: &Vec<Vec<Square>>,
    white_check_board: &mut Vec<Vec<bool>>,
    black_check_board: &mut Vec<Vec<bool>>,
) -> bool {
    let x_: i8 = (square.x as i8) + x;
    let y_: i8 = (square.y as i8) + y;

    if x_ <= 7 && x_ >= 0 && y_ >= 0 && y_ <= 7 {
        match square.piece.piece_type {
            PieceType::Pawn => {
                if (y != 0 && square.piece.white) {
                    white_check_board[x_ as usize][y_ as usize] = true;
                } else if (y != 0) {
                    black_check_board[x_ as usize][y_ as usize] = true;
                }
                let check_square: &Square = &board[x_ as usize][y_ as usize];
                if (y == 0 && !check_square.occupied) {
                    return true;
                }else if y == 0 {
                    return false;
                }
                if board[x_ as usize][y_ as usize].occupied {
                    if square.piece.white == check_square.piece.white {
                        return false;
                    }

                    match check_square.piece.piece_type {
                        PieceType::King => return false,
                        _ => return true,
                    }
                }
                return false;
            }
            PieceType::King => {
                if square.piece.white && black_check_board[x_ as usize][y_ as usize] {
                    return false;
                }
                if !square.piece.white && white_check_board[x_ as usize][y_ as usize] {
                    return false;
                }
                if square.piece.white {
                    white_check_board[x_ as usize][y_ as usize] = true;
                } else {
                    black_check_board[x_ as usize][y_ as usize] = true;
                }
                let check_square: &Square = &board[x_ as usize][y_ as usize];
                if board[x_ as usize][y_ as usize].occupied {
                    if square.piece.white == check_square.piece.white {
                        return false;
                    }

                    match check_square.piece.piece_type {
                        PieceType::King => return false,
                        _ => return true,
                    }
                }
                return true;
            }

            _ => {
                if square.piece.white {
                    white_check_board[x_ as usize][y_ as usize] = true;
                } else {
                    black_check_board[x_ as usize][y_ as usize] = true;
                }
                let check_square: &Square = &board[x_ as usize][y_ as usize];
                if board[x_ as usize][y_ as usize].occupied {
                    if square.piece.white == check_square.piece.white {
                        return false;
                    }

                    match check_square.piece.piece_type {
                        PieceType::King => return false,
                        _ => return true,
                    }
                }
                return true;
            }
        }
    }
    false
}

fn moves_pawn(
    square: &Square,
    board: &Vec<Vec<Square>>,
    mut white_check_board: &mut Vec<Vec<bool>>,
    mut black_check_board: &mut Vec<Vec<bool>>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;

    if square.piece.white {
        if x == 1 {
            if check_move(
                &square,
                2,
                0,
                board,
                &mut white_check_board,
                &mut black_check_board,
            ) {
                moves.push(Move { x: x + 2, y: y });
            }
        }
        if check_move(
            &square,
            1,
            -1,
            board,
            &mut white_check_board,
            &mut black_check_board,
        ) {
            moves.push(Move { x: x + 1, y: y - 1 });
        }
        if check_move(
            &square,
            1,
            0,
            board,
            &mut white_check_board,
            &mut black_check_board,
        ) {
            moves.push(Move { x: x + 1, y: y });
        }
        if check_move(
            &square,
            1,
            1,
            board,
            &mut white_check_board,
            &mut black_check_board,
        ) {
            moves.push(Move { x: x + 1, y: y + 1 });
        }
    } else {
        if x == 6 {
            if check_move(
                &square,
                -2,
                0,
                board,
                &mut white_check_board,
                &mut black_check_board,
            ) {
                moves.push(Move {
                    x: (x - 2) as u8,
                    y: y,
                });
            }
        }
        if check_move(
            &square,
            -1,
            -1,
            board,
            &mut white_check_board,
            &mut black_check_board,
        ) {
            moves.push(Move {
                x: (x - 1) as u8,
                y: (y - 1) as u8,
            });
        }
        if check_move(
            &square,
            -1,
            0,
            board,
            &mut white_check_board,
            &mut black_check_board,
        ) {
            moves.push(Move {
                x: (x - 1) as u8,
                y: y,
            });
        }
        if check_move(
            &square,
            -1,
            1,
            board,
            &mut white_check_board,
            &mut black_check_board,
        ) {
            moves.push(Move {
                x: (x - 1) as u8,
                y: (y + 1) as u8,
            });
        }
    }
    moves
}

fn moves_rook(
    square: &Square,
    board: &Vec<Vec<Square>>,
    mut white_check_board: &mut Vec<Vec<bool>>,
    mut black_check_board: &mut Vec<Vec<bool>>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x: u8 = square.x;
    let y: u8 = square.y;
    let p: i8 = -1;

    for i in 0..2 {
        let mut boolx = true;
        let mut booly = true;
        for j in 1..8 {
            let k = j * p.pow(i);

            if boolx
                && check_move(
                    &square,
                    k,
                    0 as i8,
                    &board,
                    &mut white_check_board,
                    &mut black_check_board,
                )
            {
                moves.push(Move {
                    x: (x as i8 + k) as u8,
                    y: y,
                });
            } else {
                boolx = false;
            }
            if x as i8 + k <= 7 && x as i8 + k >= 0 {
                if board[(x as i8 + k) as usize][y as usize].occupied {
                    boolx = false;
                }
            }
            if booly
                && check_move(
                    &square,
                    0,
                    k,
                    &board,
                    &mut white_check_board,
                    &mut black_check_board,
                )
            {
                moves.push(Move {
                    x: x,
                    y: (y as i8 + k) as u8,
                });
            } else {
                booly = false;
            }
            if y as i8 + k <= 7 && y as i8 + k >= 0 {
                if board[(x as usize) as usize][(y as i8 + k) as usize].occupied {
                    booly = false;
                }
            }
        }
    }

    moves
}

fn moves_knight(
    square: &Square,
    board: &Vec<Vec<Square>>,
    mut white_check_board: &mut Vec<Vec<bool>>,
    mut black_check_board: &mut Vec<Vec<bool>>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;
    let p: i8 = -1;

    for i in 0..2 {
        for j in 0..2 {
            let x_: i8 = 2 * p.pow(i);
            let y_: i8 = p.pow(j);
            if check_move(
                &square,
                x_,
                y_,
                &board,
                &mut white_check_board,
                &mut black_check_board,
            ) {
                moves.push(Move {
                    x: (x as i8 + x_) as u8,
                    y: (y as i8 + y_) as u8,
                });
            }
        }
    }

    for i in 0..2 {
        for j in 0..2 {
            let x_: i8 = p.pow(i);
            let y_: i8 = 2 * p.pow(j);
            if check_move(
                &square,
                x_,
                y_,
                &board,
                &mut white_check_board,
                &mut black_check_board,
            ) {
                moves.push(Move {
                    x: (x as i8 + x_) as u8,
                    y: (y as i8 + y_) as u8,
                });
            }
        }
    }
    moves
}

fn moves_bishop(
    square: &Square,
    board: &Vec<Vec<Square>>,
    mut white_check_board: &mut Vec<Vec<bool>>,
    mut black_check_board: &mut Vec<Vec<bool>>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;
    let p: i8 = -1;

    for i in 0..2 {
        for j in 0..2 {
            for k in 1..8 {
                let x_ = k * p.pow(i);
                let y_ = k * p.pow(j);
                if check_move(
                    &square,
                    x_,
                    y_,
                    &board,
                    &mut white_check_board,
                    &mut black_check_board,
                ) {
                    moves.push(Move {
                        x: (x as i8 + x_) as u8,
                        y: (y as i8 + y_) as u8,
                    });
                } else {
                    break;
                }
                if x as i8 + x_ <= 7 && x as i8 + k >= 0 && y as i8 + y_ >= 0 && y as i8 + y_ <= 7 {
                    if board[(x as i8 + x_) as usize][(y as i8 + y_) as usize].occupied {
                        break;
                    }
                }
            }
        }
    }
    moves
}

fn moves_queen(
    square: &Square,
    board: &Vec<Vec<Square>>,
    mut white_check_board: &mut Vec<Vec<bool>>,
    mut black_check_board: &mut Vec<Vec<bool>>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;
    let p: i8 = -1;

    for i in 0..2 {
        for j in 0..2 {
            for k in 1..8 {
                let x_ = k * p.pow(i);
                let y_ = k * p.pow(j);
                if check_move(
                    &square,
                    x_,
                    y_,
                    &board,
                    &mut white_check_board,
                    &mut black_check_board,
                ) {
                    moves.push(Move {
                        x: (x as i8 + x_) as u8,
                        y: (y as i8 + y_) as u8,
                    });
                } else {
                    break;
                }
                if x as i8 + x_ <= 7 && x as i8 + x_ >= 0 && y as i8 + y_ >= 0 && y as i8 + y_ <= 7
                {
                    if board[(x as i8 + x_) as usize][(y as i8 + y_) as usize].occupied {
                        break;
                    }
                }
            }
        }
    }

    for i in 0..2 {
        let mut boolx = true;
        let mut booly = true;
        for j in 1..8 {
            let k_ = j * p.pow(i);
            if boolx
                && check_move(
                    &square,
                    k_,
                    0,
                    &board,
                    &mut white_check_board,
                    &mut black_check_board,
                )
            {
                moves.push(Move {
                    x: (x as i8 + k_) as u8,
                    y: y,
                });
            } else {
                boolx = false;
            }
            if booly
                && check_move(
                    &square,
                    0,
                    k_,
                    &board,
                    &mut white_check_board,
                    &mut black_check_board,
                )
            {
                moves.push(Move {
                    x: x,
                    y: (y as i8 + k_) as u8,
                });
            } else {
                booly = false;
            }
        }
    }
    moves
}

fn moves_king(
    square: &Square,
    board: &Vec<Vec<Square>>,
    mut white_check_board: &mut Vec<Vec<bool>>,
    mut black_check_board: &mut Vec<Vec<bool>>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;

    for i in -1i8..2 {
        if check_move(
            &square,
            i,
            1,
            &board,
            &mut white_check_board,
            &mut black_check_board,
        ) {
            moves.push(Move {
                x: (x as i8 + i) as u8,
                y: y + 1,
            });
        }
        if check_move(
            &square,
            i,
            -1,
            &board,
            &mut white_check_board,
            &mut black_check_board,
        ) {
            moves.push(Move {
                x: (x as i8 + i) as u8,
                y: y + 1,
            });
        }
    }

    if check_move(
        &square,
        1,
        0,
        &board,
        &mut white_check_board,
        &mut black_check_board,
    ) {
        moves.push(Move { x: x + 1, y: y });
    }
    if check_move(
        &square,
        -1,
        0,
        &board,
        &mut white_check_board,
        &mut black_check_board,
    ) {
        moves.push(Move { x: x - 1, y: y });
    }
    moves
}

fn build_board() -> Vec<Vec<Square>> {
    let mut board: Vec<Vec<Square>> = Vec::new();
    for i in 0..8 {
        board.push(build_line(i));
    }
    board
}

fn build_line(line: u8) -> Vec<Square> {
    let mut line_vec: Vec<Square> = Vec::new();
    if line == 0 {
        line_vec.push(Square {
            x: 0,
            y: 0,
            piece: Piece {
                piece_type: PieceType::Rook,
                white: true,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 0,
            y: 1,
            piece: Piece {
                piece_type: PieceType::Knight,
                white: true,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 0,
            y: 2,
            piece: Piece {
                piece_type: PieceType::Bishop,
                white: true,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 0,
            y: 3,
            piece: Piece {
                piece_type: PieceType::Queen,
                white: true,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 0,
            y: 4,
            piece: Piece {
                piece_type: PieceType::King,
                white: true,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 0,
            y: 5,
            piece: Piece {
                piece_type: PieceType::Bishop,
                white: true,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 0,
            y: 6,
            piece: Piece {
                piece_type: PieceType::Knight,
                white: true,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 0,
            y: 7,
            piece: Piece {
                piece_type: PieceType::Rook,
                white: true,
                moves: Vec::new(),
            },
            occupied: true,
        });
    } else if line == 1 {
        for i in 0..8 {
            line_vec.push(Square {
                x: 1,
                y: i,
                piece: Piece {
                    piece_type: PieceType::Pawn,
                    white: true,
                    moves: Vec::new(),
                },
                occupied: true,
            });
        }
    } else if line >= 2 && line <= 5 {
        for i in 0..8 {
            line_vec.push(Square {
                x: line,
                y: i,
                piece: Piece {
                    piece_type: PieceType::Unoccupied,
                    white: true,
                    moves: Vec::new(),
                },
                occupied: false,
            });
        }
    } else if line == 6 {
        for i in 0..8 {
            line_vec.push(Square {
                x: 6,
                y: i,
                piece: Piece {
                    piece_type: PieceType::Pawn,
                    white: false,
                    moves: Vec::new(),
                },
                occupied: true,
            });
        }
    } else if line == 7 {
        line_vec.push(Square {
            x: 7,
            y: 0,
            piece: Piece {
                piece_type: PieceType::Rook,
                white: false,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 7,
            y: 1,
            piece: Piece {
                piece_type: PieceType::Knight,
                white: false,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 7,
            y: 2,
            piece: Piece {
                piece_type: PieceType::Bishop,
                white: false,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 7,
            y: 3,
            piece: Piece {
                piece_type: PieceType::Queen,
                white: false,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 7,
            y: 4,
            piece: Piece {
                piece_type: PieceType::King,
                white: false,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 7,
            y: 5,
            piece: Piece {
                piece_type: PieceType::Bishop,
                white: false,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 7,
            y: 6,
            piece: Piece {
                piece_type: PieceType::Knight,
                white: false,
                moves: Vec::new(),
            },
            occupied: true,
        });
        line_vec.push(Square {
            x: 7,
            y: 7,
            piece: Piece {
                piece_type: PieceType::Rook,
                white: false,
                moves: Vec::new(),
            },
            occupied: true,
        });
    }
    return line_vec;
}
