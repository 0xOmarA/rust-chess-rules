/// Represents the class that a specific chess piece belongs to
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PieceClass {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

/// Represents the two teams which can exist in a game of chess
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Team {
    Black,
    White,
}

/// Represents a chess piece belonging to a specific team
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    class: PieceClass,
    team: Team,
    number_of_moves: u8,
}

impl Piece {
    pub fn new(piece_class: PieceClass, piece_team: Team) -> Self {
        Self {
            class: piece_class,
            team: piece_team,
            number_of_moves: 0,
        }
    }

    pub fn class(&self) -> PieceClass {
        self.class
    }

    pub fn team(&self) -> Team {
        self.team
    }

    pub fn number_of_moves(&self) -> u8 {
        self.number_of_moves
    }

    pub fn is_first_move(&self) -> bool {
        return self.number_of_moves == 0;
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.class {
            PieceClass::King => match self.team {
                Team::Black => write!(f, "{}", "♚"),
                Team::White => write!(f, "{}", "♔"),
            },
            PieceClass::Queen => match self.team {
                Team::Black => write!(f, "{}", "♛"),
                Team::White => write!(f, "{}", "♕"),
            },
            PieceClass::Rook => match self.team {
                Team::Black => write!(f, "{}", "♜"),
                Team::White => write!(f, "{}", "♖"),
            },
            PieceClass::Bishop => match self.team {
                Team::Black => write!(f, "{}", "♝"),
                Team::White => write!(f, "{}", "♗"),
            },
            PieceClass::Knight => match self.team {
                Team::Black => write!(f, "{}", "♞"),
                Team::White => write!(f, "{}", "♘"),
            },
            PieceClass::Pawn => match self.team {
                Team::Black => write!(f, "{}", "♟"),
                Team::White => write!(f, "{}", "♙"),
            },
        }
    }
}
