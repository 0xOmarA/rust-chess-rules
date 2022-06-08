use crate::coordinate::Coordinate;
use crate::board::Fen;

mod board;
mod coordinate;
mod piece;

fn main() {
    let mut board: board::Board = board::Board::new_with_fen(Fen { state: "8/8/8/4p1K1/2k1P3/8/8/8 b - - 0 1".to_string() });
    
    // let from: Coordinate = Coordinate::try_from("E2").unwrap();
    // let to: Coordinate = Coordinate::try_from("E4").unwrap();
    // println!("Response is: {:?}", board.move_piece(&from, &to));
    
    println!("{}", board);
    let fen: Fen = board.fen();
    println!("{:?}", fen);
}