# simonsev-chess
This is my chess API...

## Usage
To start off you'll want to create a new game instance through:
```rust
let mut game = Game::new();
```
This creates a new game with all pieces in their standard position, all possible moves generated and the turn set to white.

To take a turn, you will first want to input the next move by using:
```rust
game.input_move(from, to);
```
where "from" is a string representing the square which you want to perform a move from, and "to" being a string representing the square you want the piece on "from" to move to.

Subsequently, ```rust do_turn()``` will perform all necessary events that constitutes a complete turn, assuming that the move is valid, which is also checked in ```rust do_turn()```.

So, creating a new game and moving the E2 pawn to E3 would look like this:
```rust 
let mut game = Game::new();
game.input_move("E2", "E3");
game = game.do_turn();
```
, that's it!

As i mentioned, ```rust do_turn()``` will check if the move is valid, if not it will simply reset ```rust move_from```, ```rust move_to``` and return without doing anything else.

Now, of course, betwixt one move and the next, you'll want to check for whether mate has been made.
This can simply be done by accessing the "mate" boolean in your ```rust Game``` instance, as ```rust do_turn()``` automatically checks for mate at the end of each turn. When mate is found, the turn will not progress to the other player. This means that who has mated whom can be determined by checking the value of "white_turn" in your ```rust Game``` instance. If the value is true then white has mated black and vice versa.

To access the board you'll want to use ```rust game.get_board()```, which returns a clone of "board" contained within "game". This is an 8x8 2D vec containing ```rust Square``` instances.


## The parts.
This API is made up of a few different pieces, that is, 5 structs, 1 enum and a whole lot of functions. It may seem a bit convoluted at first, but you need not pay attention to most of it when using the API. You will however need to have some understanding of what is actually contained within the structs in order to paint the board. The general structure of a game can be visualized as follows:

struct Game 
     &#8595;
struct Boards
     &#8595;
struct Square
     &#8595;
struct Piece
     &#8595;
enum PieceType

and the struct ```rust Move``` is just a tool to smoothify passing positions and moves through functions.

### The PieceType enum.
This is simply an enum containing all the different types a chess piece can have, including unoccupied.

### The Piece struct.
This struct is representative of a piece. As such it contains a ```rust PieceType```, a ```rust bool``` signifying its color and a ```rust Vec<Move>```, containing all it's valid moves on a given turn.

### The Square struct.
```rust Square``` represents a square on the board. It contains an ```rust x``` and ```rust y``` of type ```rust u8```, both ranging from 0 to 7, signifying it's position on the board (x is height and y is width, sorry). It also contains a ```rust Piece``` and a ```rust bool``` for whether its occupied by a piece or not (yes there is a redundnacy between this boolean and the 'Unoccupied' value in PieceType, let it be). 

### The Boards struct.
This is probably the simplest struct in the API, it contains three 8x8 2-D vectors: "board" of type ```rust Vec<Vec<Square>>```, "white_check_board" of type ```rust Vec<Vec<bool>>``` and "black_check_board" of the same type. 

"board" is the actual playing field, this vector is how the API tracks all the pieces and it is also used to check what moves each piece can do.
A clone of this vector is also what you will be using to paint the board.

"white_check_board" and "black_check_board" keeps track of all different squares that the respective player could move to, using any piece, on a turn. They are as such used mostly to check for check and mate.

### The Game struct.
This is the top struct in the API and looks as follows:
```rust
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
```
The ```rust bool``` "white_turn" simply tracks whose turn it is, true means white's turn, and vice versa.

"w_king_pos" and "b_king_pos" tracks white's and black's king position, respectively.

Move history has not yet been implemented so "move_history" is just sitting there for now.

"move_from" and "move_to" start as empty strings, this is where ```rust input_move(from, to)``` goes, they are emptied after each attempted and completed turn.

"mate" tracks whether mate has been reached or not, if mate is found it will be set to true.
