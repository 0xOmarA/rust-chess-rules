use crate::coordinate::Coordinate;

mod board;
mod coordinate;
mod piece;

fn main() {
    let mut board: board::Board = board::Board::new();

    let from: Coordinate = Coordinate::try_from("E2").unwrap();
    let to: Coordinate = Coordinate::try_from("E4").unwrap();
    println!("Response is: {:?}", board.move_piece(&from, &to));
    
    let from: Coordinate = Coordinate::try_from("E4").unwrap();
    let to: Coordinate = Coordinate::try_from("E5").unwrap();
    println!("Response is: {:?}", board.move_piece(&from, &to));
    
    let from: Coordinate = Coordinate::try_from("E5").unwrap();
    let to: Coordinate = Coordinate::try_from("E6").unwrap();
    println!("Response is: {:?}", board.move_piece(&from, &to));
    
    let from: Coordinate = Coordinate::try_from("E7").unwrap();
    let to: Coordinate = Coordinate::try_from("E5").unwrap();
    println!("Response is: {:?}", board.move_piece(&from, &to));
    println!("{}", board);
}