use std::cmp::PartialEq;

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

pub fn square_to_unoccupied(pos: &Move, board: &mut Vec<Vec<Square>>) {
    board[pos.x as usize][pos.y as usize].piece.piece_type = PieceType::Unoccupied;
    board[pos.x as usize][pos.y as usize].piece.white = true;
    board[pos.x as usize][pos.y as usize].piece.moves = Vec::new();
    board[pos.x as usize][pos.y as usize].occupied = false;
}

pub fn generate_moves(x: usize, y: usize, board: &mut Vec<Vec<Square>>) {
    match board[x as usize][y as usize].piece.piece_type {
        PieceType::Pawn => {
            board[x as usize][y as usize].piece.moves =
                moves_pawn(&board[x as usize][y as usize], board)
        }
        PieceType::Rook => {
            board[x as usize][y as usize].piece.moves =
                moves_rook(&board[x as usize][y as usize], board)
        }
        PieceType::Knight => {
            board[x as usize][y as usize].piece.moves =
                moves_knight(&board[x as usize][y as usize], board)
        }
        PieceType::Bishop => {
            board[x as usize][y as usize].piece.moves =
                moves_bishop(&board[x as usize][y as usize], board)
        }
        PieceType::Queen => {
            board[x as usize][y as usize].piece.moves =
                moves_queen(&board[x as usize][y as usize], board)
        }
        PieceType::King => {
            board[x as usize][y as usize].piece.moves =
                moves_king(&board[x as usize][y as usize], board)
        }
        _ => (),
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
                .piece_type = PieceType::Queen
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

pub struct Piece {
    pub piece_type: PieceType,
    pub white: bool,
    pub moves: Vec<Move>,
}

#[derive(PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Unoccupied,
}

#[derive(PartialEq, Eq)]
pub struct Move {
    pub x: u8,
    pub y: u8,
}

impl Move {
    pub fn move_to_string(&self) -> String {
        let mut s = String::new();
        s.push((&self.x + 65) as char);
        s.push((&self.y + 49) as char);

        s
    }

    pub fn print_move(&self) {
        print!("{:?},", &self.x);
        println!("{:?}", &self.y);
    }
}

pub fn string_to_move(input: &String) -> Move {
    let mut x: &char = &input.chars().nth(0).unwrap();
    let mut y: &char = &input.chars().nth(1).unwrap();

    let mut x = x.to_owned() as u8;
    let mut y = y.to_owned() as u8;

    x -= 65;
    y -= 49;

    Move { x: x, y: y }
}

fn check_move(square: &Square, x: i8, y: i8, board: &Vec<Vec<Square>>) -> bool {
    let x_: i8 = (square.x as i8) + x;
    let y_: i8 = (square.y as i8) + y;

    if x_ <= 7 && x_ >= 0 && y_ >= 0 && y_ <= 7 {
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
    false
}

pub fn moves_pawn(square: &Square, board: &Vec<Vec<Square>>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;

    if check_move(&square, 1, -1, board) {
        moves.push(Move { x: x + 1, y: y - 1 });
    }
    if check_move(&square, 1, 0, board) {
        moves.push(Move { x: x + 1, y: y });
    }
    if check_move(&square, 1, 1, board) {
        moves.push(Move { x: x + 1, y: y + 1 });
    }

    moves
}

fn moves_rook(square: &Square, board: &Vec<Vec<Square>>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x: u8 = square.x;
    let y: u8 = square.y;
    let p: i8 = -1;

    for i in 0..2 {
        for j in 0..8 {
            let k = j * p.pow(i);
            let x_ = x as i8 + k;
            let y_ = y as i8 + k;

            if check_move(&square, x_, y as i8, &board) {
                moves.push(Move { x: x_ as u8, y: y});
            }
            if check_move(&square, x as i8, y_, &board) {
                moves.push(Move { x: x, y: y_ as u8});
            }
        }
    }
/* 
    if square.piece.white {
        for i in (x + 1)..8 {
            let sq_forward: &Square = &board[x as usize][i as usize];
            let piece_forward: &Piece = &sq_forward.piece;
            if piece_forward.white {
                match piece_forward.piece_type {
                    PieceType::Unoccupied => moves.push(Move { x: i, y: y }),
                    _ => break,
                }
            } else {
                match piece_forward.piece_type {
                    PieceType::King => break,
                    _ => {
                        moves.push(Move { x: i, y: y });
                        break;
                    }
                }
            }
        }

        for i in (0..(x - 1)).rev() {
            let sq_check: &Square = &board[i as usize][x as usize];
            let piece_check: &Piece = &sq_check.piece;
            if piece_check.white {
                match piece_check.piece_type {
                    PieceType::Unoccupied => moves.push(Move { x: i, y: y }),
                    _ => break,
                }
            } else {
                match piece_check.piece_type {
                    PieceType::King => break,
                    _ => {
                        moves.push(Move { x: i, y: y });
                        break;
                    }
                }
            }
        }

        for i in (y + 1)..8 {
            let sq_check: &Square = &board[x as usize][i as usize];
            let piece_check: &Piece = &sq_check.piece;
            if piece_check.white {
                match piece_check.piece_type {
                    PieceType::Unoccupied => moves.push(Move { x: x, y: i }),
                    _ => break,
                }
            } else {
                match piece_check.piece_type {
                    PieceType::King => break,
                    _ => {
                        moves.push(Move { x: x, y: i });
                        break;
                    }
                }
            }
        }
    }*/
    moves
}

pub fn moves_knight(square: &Square, board: &Vec<Vec<Square>>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;
    let p: i8 = -1;

    for i in 0..2 {
        for j in 0..2 {
            let x_: i8 = 2 * p.pow(i);
            let y_: i8 = p.pow(j);
            if check_move(&square, x_, y_, &board) {
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
            if check_move(&square, x_, y_, &board) {
                moves.push(Move {
                    x: (x as i8 + x_) as u8,
                    y: (y as i8 + y_) as u8,
                });
            }
        }
    }
    moves
}

fn moves_bishop(square: &Square, board: &Vec<Vec<Square>>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;
    let p: i8 = -1;

    for i in 0..2 {
        for j in 0..2 {
            for k in 1..8 {
                let x_ = k * p.pow(i);
                let y_ = k * p.pow(j);
                if !check_move(&square, x_, y_, &board) {
                    break;
                } else {
                    moves.push(Move {
                        x: (x as i8 + x_) as u8,
                        y: (y as i8 + y_) as u8,
                    });
                    if board[(x as i8 + x_) as usize][(y as i8 + y_) as usize].occupied {
                        break;
                    }
                }
            }
        }
    }
    moves
}

fn moves_queen(square: &Square, board: &Vec<Vec<Square>>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;
    let p: i8 = -1;

    for i in 0..2 {
        for j in 0..2 {
            for k in 1..8 {
                let x_ = k * p.pow(i);
                let y_ = k * p.pow(j);
                if check_move(&square, x_, y_, &board) {
                    moves.push(Move {
                        x: (x as i8 + x_) as u8,
                        y: (y as i8 + y_) as u8,
                    });
                }
            }
        }
    }

    for i in 0..2 {
        for j in 1..8 {
            let k_ = j * p.pow(i);
            if check_move(&square, k_, 0, &board) {
                moves.push(Move {
                    x: (x as i8 + k_) as u8,
                    y: y,
                });
            }
            if check_move(&square, 0, y as i8 + k_, &board) {
                moves.push(Move {
                    x: x,
                    y: (y as i8 + k_) as u8,
                });
            }
        }
    }
    moves
}

fn moves_king(square: &Square, board: &Vec<Vec<Square>>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let x = square.x;
    let y = square.y;

    for i in -1i8..2 {
        if check_move(&square, (x as i8 + i), y as i8 + 1, &board) {
            moves.push(Move {
                x: (x as i8 + i) as u8,
                y: y + 1,
            });
        }
        if check_move(&square, (x as i8 + i), y as i8 - 1, &board) {
            moves.push(Move {
                x: (x as i8 + i) as u8,
                y: y + 1,
            });
        }
    }

    if check_move(&square, x as i8 + 1, y as i8, &board) {
        moves.push(Move { x: x + 1, y: y });
    }
    if check_move(&square, x as i8 - 1, y as i8, &board) {
        moves.push(Move { x: x - 1, y: y });
    }
    moves
}
