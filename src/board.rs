use crate::coordinate::{Coordinate, CoordinatePath};
use crate::piece::{Piece, PieceClass, Team};
use itertools::Itertools;
use std::collections::HashMap;

/// Represents the current chess board with all of its pieces
#[derive(Debug)]
pub struct Board {
    /// A two dimensional array of the actual board.
    map: [[Option<Piece>; 8]; 8],

    /// A graveyard for all of the pieces which have been removed.
    graveyard: Vec<Piece>,
}

impl Board {
    /// Creates a new default board
    pub fn new() -> Self {
        let mut board: Self = Self::default();

        // Setting all of the pieces fo their correct place
        for (row_index, team) in [(0, Team::Black), (7, Team::White)] {
            for (i, item_class) in [
                PieceClass::Rook,
                PieceClass::Knight,
                PieceClass::Bishop,
                PieceClass::King,
                PieceClass::Queen,
                PieceClass::Bishop,
                PieceClass::Knight,
                PieceClass::Rook,
            ]
            .iter()
            .enumerate()
            {
                board.set_piece(
                    &Coordinate::try_from((row_index, i)).unwrap(),
                    Some(Piece::new(item_class.clone(), team)),
                );
            }
        }

        for (row_index, team) in [(1, Team::Black), (6, Team::White)] {
            for i in 0u8..8u8 {
                board.set_piece(
                    &Coordinate::try_from((row_index, i)).unwrap(),
                    Some(Piece::new(PieceClass::Pawn, team)),
                );
            }
        }

        board
    }

    fn remove_piece(&mut self, coordinate: &Coordinate) -> Result<(), BoardError> {
        let piece: Option<Piece> = self.get_piece(coordinate);

        match piece {
            Some(piece) => {
                self.graveyard.push(piece);
                self.set_piece(coordinate, None);
                Ok(())
            }
            None => Err(BoardError::EmptyCoordinate),
        }
    }

    fn set_piece(&mut self, coordinate: &Coordinate, piece: Option<Piece>) {
        self.map[coordinate.row()][coordinate.column()] = piece;
    }

    pub fn get_piece(&self, coordinate: &Coordinate) -> Option<Piece> {
        self.map[coordinate.row()][coordinate.column()]
    }

    pub fn map(&self) -> [[Option<Piece>; 8]; 8] {
        self.map
    }

    /// Moves a piece from one coordinate to another coordinate. Checks that the move is legal before performing the 
    /// move.
    pub fn move_piece(
        &mut self,
        from: &Coordinate,
        to: &Coordinate
    ) -> Result<(), BoardError> {
        // Getting the piece at the specified coordinate.
        let piece: Piece = {
            match self.get_piece(from) {
                Some(piece) => Ok(piece),
                None => Err(BoardError::EmptyCoordinate),
            }
        }?;

        // Getting all of the legal moves for this piece
        println!("Before legal move calculation");
        let legal_moves: HashMap<Coordinate, Option<Coordinate>> = self.piece_legal_moves(from).unwrap();
        println!("Legal moves are: {:?}", legal_moves);

        // If true, then this is a legal move and we can go ahead with the removal of the old item.
        if legal_moves.contains_key(to) {
            // If there is an item to destroy, go ahead and destroy it.
            match legal_moves.get(from) {
                Some(to_destroy_coordinate) => {
                    self.remove_piece(&to_destroy_coordinate.unwrap())?;
                },
                None => {}
            }

            // Perform the move operation
            self.set_piece(to, Some(piece));
            self.set_piece(from, None);

            Ok(())
        } else {
            println!("Does not contain key");
            Err(BoardError::IllegalMove)
        }
    }

    /// This method gets all of the legal moves that a specific piece from a specific coordinate is allowed to make and
    /// returns it as well as the coordinate of the items to be removed if the piece is moved to this coordinate.
    ///
    /// Therefore, the HashMap returned is:
    ///
    /// HashMap<Coordinate, Option<Coordinate>
    ///              │                 │
    ///              │                 └ If the move is made, this piece will be removed in the process.
    ///              └ A coordinate that the piece is allowed to move to
    pub fn piece_legal_moves(
        &self,
        coordinate: &Coordinate,
    ) -> Result<HashMap<Coordinate, Option<Coordinate>>, BoardError> {
        // Getting the piece at the specified coordinate.
        let piece: Piece = {
            match self.get_piece(coordinate) {
                Some(piece) => Ok(piece),
                None => Err(BoardError::EmptyCoordinate),
            }
        }?;

        // This is the HashMap that will be returned. The following match statement will only modify this HashMap, it
        // wont create a new one.
        let mut legal_moves: HashMap<Coordinate, Option<Coordinate>> = HashMap::new();

        // The logic depends on the type of the piece that we're checking for.
        match piece.class() {
            PieceClass::Knight => {
                // The night only has a set of coordinates that they can move to, nothing else. Here we calculate the
                // possible coordinate offsets that they can move to.
                let coordinates: Vec<Coordinate> = vec![1, 2, -1, -2]
                    .iter()
                    .cloned()
                    .permutations(2)
                    .filter(|perm| perm.iter().map(|n| i8::abs(*n)).sum::<i8>() == 3)
                    .map(|offsets| {
                        let row_offset: i8 = offsets[0];
                        let column_offset: i8 = offsets[1];

                        coordinate.checked_add_individual(row_offset, column_offset)
                    })
                    .filter(|maybe_coordinate| maybe_coordinate.is_ok())
                    .map(|x| x.unwrap())
                    .collect();

                // Go over the coordinates and ensure that the knight can only move to coordinates where no friendlies
                // are
                for single_coordinate in coordinates.into_iter() {
                    match self.get_piece(&single_coordinate) {
                        Some(other_piece) => {
                            if other_piece.team() != piece.team() {
                                legal_moves.insert(
                                    single_coordinate.clone(),
                                    Some(single_coordinate.clone()),
                                );
                            }
                        }
                        None => {
                            legal_moves.insert(single_coordinate, None);
                        }
                    }
                }
            }
            // These pieces will all follow the same logic of CoordinatePaths but will be using different paths. So, it
            // is better to group their logic into one single place to be able to use it quickly.
            PieceClass::Bishop | PieceClass::Rook | PieceClass::Queen | PieceClass::King => {
                let paths: Vec<CoordinatePath> = {
                    match piece.class() {
                        PieceClass::Bishop => vec![-1, -1, 1, 1]
                            .iter()
                            .cloned()
                            .permutations(2)
                            .unique()
                            .map(|multipliers| {
                                let row_multiplier: i8 = multipliers[0];
                                let column_multiplier: i8 = multipliers[1];

                                (1..8)
                                    .map(|n| {
                                        (n.clone() * row_multiplier, n.clone() * column_multiplier)
                                    })
                                    .collect::<Vec<(i8, i8)>>()
                            })
                            .map(|offsets_vec| {
                                offsets_vec
                                    .iter()
                                    .map(|(row_offset, column_offset)| {
                                        coordinate
                                            .checked_add_individual(*row_offset, *column_offset)
                                    })
                                    .filter(|maybe_coordinate| maybe_coordinate.is_ok())
                                    .map(|x| x.unwrap())
                                    .collect()
                            })
                            .collect::<Vec<CoordinatePath>>(),
                        PieceClass::Rook => vec![-1, 1]
                            .iter()
                            .cloned()
                            .flat_map(|multiplier| {
                                vec![
                                    (1..8)
                                        .map(|n| (0, n.clone() * multiplier))
                                        .collect::<Vec<(i8, i8)>>(),
                                    (1..8)
                                        .map(|n| (n.clone() * multiplier, 0))
                                        .collect::<Vec<(i8, i8)>>(),
                                ]
                            })
                            .map(|offsets_vec| {
                                offsets_vec
                                    .iter()
                                    .map(|(row_offset, column_offset)| {
                                        coordinate
                                            .checked_add_individual(*row_offset, *column_offset)
                                    })
                                    .filter(|maybe_coordinate| maybe_coordinate.is_ok())
                                    .map(|x| x.unwrap())
                                    .collect()
                            })
                            .collect::<Vec<CoordinatePath>>(),
                        PieceClass::Queen | PieceClass::King => {
                            let end: i8 = if matches!(piece.class(), PieceClass::Queen) {
                                8
                            } else {
                                2
                            };
                            vec![-1, 1]
                                .iter()
                                .cloned()
                                .flat_map(|multiplier| {
                                    vec![
                                        (1..end)
                                            .map(|n| (0, n.clone() * multiplier))
                                            .collect::<Vec<(i8, i8)>>(),
                                        (1..end)
                                            .map(|n| (n.clone() * multiplier, 0))
                                            .collect::<Vec<(i8, i8)>>(),
                                    ]
                                })
                                .map(|offsets_vec| {
                                    offsets_vec
                                        .iter()
                                        .map(|(row_offset, column_offset)| {
                                            coordinate
                                                .checked_add_individual(*row_offset, *column_offset)
                                        })
                                        .filter(|maybe_coordinate| maybe_coordinate.is_ok())
                                        .map(|x| x.unwrap())
                                        .collect()
                                })
                                .chain(
                                    vec![-1, -1, 1, 1]
                                        .iter()
                                        .cloned()
                                        .permutations(2)
                                        .unique()
                                        .map(|multipliers| {
                                            let row_multiplier: i8 = multipliers[0];
                                            let column_multiplier: i8 = multipliers[1];

                                            (1..end)
                                                .map(|n| {
                                                    (
                                                        n.clone() * row_multiplier,
                                                        n.clone() * column_multiplier,
                                                    )
                                                })
                                                .collect::<Vec<(i8, i8)>>()
                                        })
                                        .map(|offsets_vec| {
                                            offsets_vec
                                                .iter()
                                                .map(|(row_offset, column_offset)| {
                                                    coordinate.checked_add_individual(
                                                        *row_offset,
                                                        *column_offset,
                                                    )
                                                })
                                                .filter(|maybe_coordinate| maybe_coordinate.is_ok())
                                                .map(|x| x.unwrap())
                                                .collect()
                                        }),
                                )
                                .collect::<Vec<CoordinatePath>>()
                        }
                        _ => {
                            panic!("Impossible case occurred.")
                        }
                    }
                };

                for path in paths.into_iter() {
                    for single_coordinate in path.into_iter() {
                        match self.get_piece(&single_coordinate) {
                            Some(other_piece) => {
                                if other_piece.team() != piece.team() {
                                    legal_moves.insert(
                                        single_coordinate.clone(),
                                        Some(single_coordinate.clone()),
                                    );
                                }
                                break;
                            }
                            None => {
                                legal_moves.insert(single_coordinate, None);
                            }
                        }
                    }
                }
            }
            PieceClass::Pawn => {
                // Determine the direction of allowed movements depending on the team
                let single_pawn_move: i8 = match piece.team() {
                    Team::Black => 1,
                    Team::White => -1,
                };

                // Single pawn move
                match coordinate.checked_add_individual(single_pawn_move, 0) {
                    Ok(single_coordinate) => {
                        match self.get_piece(&single_coordinate) {
                            Some(_) => { }
                            None => {
                                legal_moves.insert(single_coordinate, None);
                            }
                        }
                    },
                    Err(_) => {}
                }

                // Two pawn move
                if piece.is_first_move() {
                    match coordinate.checked_add_individual(single_pawn_move * 2, 0) {
                        Ok(single_coordinate) => {
                            match self.get_piece(&single_coordinate) {
                                Some(_) => { }
                                None => {
                                    legal_moves.insert(single_coordinate, None);
                                }
                            }
                        },
                        Err(_) => {}
                    }
                }

                // Pawn's attack move
                for column_offset in [-1, 1] {
                    match coordinate.checked_add_individual(single_pawn_move, column_offset) {
                        Ok(single_coordinate) => {
                            match self.get_piece(&single_coordinate) {
                                Some(other_piece) => { 
                                    if other_piece.team() != piece.team() {
                                        legal_moves.insert(
                                            single_coordinate.clone(),
                                            Some(single_coordinate.clone()),
                                        );
                                    }
                                }
                                None => {
                                    legal_moves.insert(single_coordinate, None);
                                }
                            }
                        },
                        Err(_) => {}
                    }
                }
            } 
        }

        return Ok(legal_moves);
    }
}

impl Default for Board {
    /// Creates a new empty vault
    fn default() -> Self {
        Self {
            map: Default::default(),
            graveyard: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum BoardError {
    EmptyCoordinate,
    IllegalMove
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (row_number, row) in self.map().iter().enumerate() {
            write!(f, "{} ┃ ", 8 - row_number)?;
            for item in row.iter() {
                write!(
                    f,
                    "{} ",
                    match item {
                        Some(item) => format!("{}", item),
                        None => format!("."),
                    }
                )?
            }
            write!(f, "\n")?;
        }
        write!(f, "  ┗━━━━━━━━━━━━━━━━\n")?;
        write!(f, "    ")?;
        for letter in 'A'..'I' {
            write!(f, "{} ", letter)?;
        }
        std::fmt::Result::Ok(())
    }
}
