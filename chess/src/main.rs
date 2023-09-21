mod utils;
use utils::*;

fn main() {

}

fn move_piece(from: &String, to: &String, mut board: Vec<Vec<Square>>) -> Vec<Vec<Square>> {
    let mut move_from = string_to_move(&from);
    let mut move_to = string_to_move(&to);

    if board[move_from.x as usize][move_from.y as usize]
        .piece
        .moves
        .contains(&move_to)
    {
        println!("move found");
        square_to_square(&move_from, move_to, &mut board);
        square_to_unoccupied(&move_from, &mut board);
    } else {
        println!("Move not found");
    }

    board
}

fn print_board(mut board: &Vec<Vec<Square>>) {
    for i in (0..board.len()).rev() {
        for j in 0..board[i].len() {
            board[i][j].print_square();
        }
        println!("");
    }
}

fn find_all_moves(mut board: Vec<Vec<Square>>) -> Vec<Vec<Square>> {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j].occupied {
                generate_moves(i, j, &mut board);
            }
        }
    }
    board
}

fn reset_check_board(mut check_board: Vec<Vec<bool>>) {
    for i in 0..check_board.len() {
        for j in 0..check_board[i].len() {
            check_board[i][j] = false;
        }
    }
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
                x: 1,
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
