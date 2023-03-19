use super::SpielBrett;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum RichtungHorizontal {
    Links,
    Rechts,
}

impl RichtungHorizontal {
    pub fn offset(self) -> isize {
        match self {
            Self::Links => -1,
            Self::Rechts => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum RichtungVertikal {
    Oben,
    Unten,
}

impl RichtungVertikal {
    pub fn offset(self) -> isize {
        match self {
            Self::Oben => -1,
            Self::Unten => 1,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Position {
    pub spalte: usize,
    pub zeile: usize,
}

impl Position {
    pub fn valid_spalte(spalte: isize) -> bool {
        spalte >= 0 && (spalte as usize) < SpielBrett::SIZE
    }

    pub fn valid_zeile(zeile: isize) -> bool {
        zeile >= 0 && (zeile as usize) < SpielBrett::SIZE
    }

    pub fn valid(self) -> bool {
        self.spalte < SpielBrett::SIZE
            && self.zeile < SpielBrett::SIZE
            && self.spalte % 2 == self.zeile % 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_valid() {
        macro_rules! valid {
            ($spalte: literal $zeile: literal $valid: literal) => {
                assert_eq!(
                    Position {
                        spalte: $spalte,
                        zeile: $zeile
                    }
                    .valid(),
                    $valid
                );
            };
        }

        valid!(0 0 true);
        valid!(0 1 false);
        valid!(0 2 true);
        valid!(0 3 false);
        valid!(0 4 true);
        valid!(0 5 false);
        valid!(0 6 true);
        valid!(0 7 false);

        valid!(1 0 false);
        valid!(1 1 true);
        valid!(1 2 false);
        valid!(1 3 true);
        valid!(1 4 false);
        valid!(1 5 true);
        valid!(1 6 false);
        valid!(1 7 true);

        valid!(2 0 true);
        valid!(2 1 false);
        valid!(2 2 true);
        valid!(2 3 false);
        valid!(2 4 true);
        valid!(2 5 false);
        valid!(2 6 true);
        valid!(2 7 false);

        valid!(3 0 false);
        valid!(3 1 true);
        valid!(3 2 false);
        valid!(3 3 true);
        valid!(3 4 false);
        valid!(3 5 true);
        valid!(3 6 false);
        valid!(3 7 true);

        valid!(8 0 false);
        valid!(0 8 false);
    }
}
