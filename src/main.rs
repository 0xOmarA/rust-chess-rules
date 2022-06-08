use crate::coordinate::Coordinate;

mod board;
mod coordinate;
mod piece;

fn main() {
    let mut board: board::Board = board::Board::new();

    let from: Coordinate = Coordinate::try_from("H2").unwrap();
    let to: Coordinate = Coordinate::try_from("H5").unwrap();

    println!("Response is: {:?}", board.move_piece(&from, &to));
    println!("{}", board);
}