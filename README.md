# Pleco

Pleco is a chess Engine & Library derived from Stockfish, written entirely in Rust.

[![Pleco crate](https://img.shields.io/crates/v/pleco.svg)](https://crates.io/crates/pleco)
[![Pleco crate](https://img.shields.io/crates/v/pleco_engine.svg)](https://crates.io/crates/pleco_engine)
[![Build Status](https://api.travis-ci.org/sfleischman105/Pleco.svg?branch=master)](https://travis-ci.org/sfleischman105/Pleco)


This project is split into two crates, `pleco`, which contains the library functionality, and `pleco_engine`, which contains the
UCI (Universal Chess Interface) compatible engine. 

The overall goal for this project is to utilize the efficiency of Rust to create a Chess AI matching the speed of modern chess engines.
For the engine, the majority of the code is a direct port of Stockfish's C++ code. See [their website](https://stockfishchess.org/) for
more information about the engine. As such, the credit for all of the advanced algorithms used for searching, evaluation,
and many others, go directly to the maintainers and authors of Stockfish. This project is for speed comparisons
between the two languages, as well as for educational purposes.

- [Documentation](https://docs.rs/pleco), [crates.io](https://crates.io/crates/pleco) for library functionality
- [Documentation](https://docs.rs/pleco_engine), [crates.io](https://crates.io/crates/pleco_engine) for the Engine.

Standalone Installation and Use
-------

To use pleco as an executable, please [navigate to here](https://github.com/sfleischman105/Pleco/tree/master/pleco_engine) and read the `README.md`. 


Using Pleco as a Library
-------

To use pleco inside your own Rust projects, [Pleco.rs is available as a library on crates.io.](https://crates.io/crates/pleco)
Simply include the current version in your `Cargo.toml`:

```
[dependencies]
pleco = "x.x.x"
```

And add the following to a `main.rs` or `lib.rs`:
```rust
extern crate pleco;
```

### Basic Usage
Setting up a board position is extremely simple.
```rust
use pleco::{Board,Player,PieceType};

let board = Board::start_pos();
assert_eq!(board.count_piece(Player::White,PieceType::P), 8);
assert_eq!(&board.fen(),"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
```

#### Creating a board from a Position
A `Board` can be created with any valid chess position using a valid FEN (Forsyth-Edwards Notation) String. 
Check out the [Wikipedia article](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) for more information on FEN Strings
and their format.

```rust
let board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2").unwrap();
```

#### Applying and Generating Moves
Moves are represented with a `BitMove` structure. They must be generated by a `Board` object directly, to be 
considered a valid move. Using `Board::generate_moves()` will generate all legal `BitMove`s of the current 
position for the current player.
```rust
use pleco::{Board,BitMove};

let mut board = Board::start_pos(); // create a board of the starting position
let moves = board.generate_moves(); // generate all possible legal moves
board.apply_move(moves[0]);
assert_eq!(board.moves_played(), 1);
```


We can ask the Board to apply a move to itself from a string. This string must follow the format of a standard
UCI Move, in the format [src_sq][dst_sq][promo]. E.g., moving a piece from A1 to B3 would have a uci string of "a1b3",
while promoting a pawn would look something like "e7e81". If the board is supplied a UCI move that is either 
incorrectly formatted or illegal, false shall be returned.
```rust
let mut board = Board::start_pos(); // create a board of the starting position
let success = board.apply_uci_move("e7e8q"); // apply a move where piece on e7 -> eq, promotes to queen
assert!(!success); // Wrong, not a valid move for the starting position
```

#### Undoing Moves
We can revert to the previous chessboard state with a simple Board::undo_move()
```rust
let mut board = Board::start_pos();
board.apply_uci_move("e2e4"); // A very good starting move, might I say
assert_eq!(board.moves_played(),1);
board.undo_move();
assert_eq!(board.moves_played(),0);
```

For more informaton about `pleco` as a library, see the [pleco README.md](https://github.com/sfleischman105/Pleco/tree/master/pleco).

Contributing
-------

Any and all contributions are welcome! Open up a PR to contribute some improvements. Look at the Issues tab to see what needs some help. 


  
License
-------
Pleco is distributed under the terms of the MIT license. See LICENSE-MIT for details. Opening a pull requests is assumed to signal agreement with these licensing terms.