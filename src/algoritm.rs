/// Pretty self explanatory, but it describes the type of move.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    Prime,
    Double,
}

/// Describes which side should be moves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Side {
    Up,
    Down,
    Right,
    Left,
    Front,
    Back,
}

/// Describes a move, both by [`MoveType`] and [`Side`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub side: Side,
    pub ty: MoveType,
}

impl Move {
    pub fn new(side: Side, ty: MoveType) -> Self {
        Self { side, ty }
    }

    pub fn parse(source: &str) -> Option<Self> {
        let side = match source.chars().nth(0)? {
            'U' => Side::Up,
            'D' => Side::Down,
            'R' => Side::Right,
            'L' => Side::Left,
            'F' => Side::Front,
            'B' => Side::Back,
            _ => return None,
        };

        let ty = match source.chars().nth(1) {
            None => MoveType::Normal,
            Some('\'') => MoveType::Prime,
            Some('2') => MoveType::Double,
            _ => return None,
        };

        Some(Self { side, ty })
    }
}

/// A set of moves, executed in sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Algorithm {
    pub moves: Vec<Move>,
}

impl Algorithm {
    pub fn new(moves: Vec<Move>) -> Self {
        Self { moves }
    }

    /// Parses from a string.
    ///
    /// ```
    /// let alg = Algorithm::parse("R U R' U'").unwrap();
    /// ```
    pub fn parse(source: &str) -> Option<Self> {
        let source = source.trim();

        let mut moves = Vec::new();

        for s in source.split_whitespace() {
            moves.push(Move::parse(s)?);
        }

        Some(Self { moves })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MoveType::*;
    use Side::*;

    #[test]
    pub fn algoritm_parsing() {
        let expected = Algorithm::new(vec![
            Move::new(Right, Normal),
            Move::new(Up, Normal),
            Move::new(Right, Prime),
            Move::new(Up, Prime),
        ]);

        let parsed = Algorithm::parse("R U R' U'");

        assert_eq!(parsed, Some(expected));
    }
}
