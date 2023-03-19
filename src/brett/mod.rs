use std::{fmt::{Debug, Formatter, self}, ops::Not};

mod position;
pub use position::*;
mod moves;
mod parse;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Spieler {
    Mensch,
    Computer,
}

impl Spieler {
    pub fn move_direction(self) -> RichtungVertikal {
        match self {
            Self::Mensch => RichtungVertikal::Unten,
            Self::Computer => RichtungVertikal::Oben,
        }
    }

    pub fn dame_zeile(self) -> usize {
        match self {
            Self::Mensch => SpielBrett::SIZE - 1,
            Self::Computer => 0,
        }
    }
}

impl Not for Spieler {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Mensch => Self::Computer,
            Self::Computer => Self::Mensch,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default, Debug)]
pub enum Feld {
    #[default]
    Leer,
    Stein(Spieler),
    Dame(Spieler),
}

impl Feld {
    pub fn to_character(self) -> char {
        match self {
            Self::Leer => '_',
            Self::Stein(Spieler::Mensch) => 'm',
            Self::Dame(Spieler::Mensch) => 'M',
            Self::Stein(Spieler::Computer) => 'c',
            Self::Dame(Spieler::Computer) => 'C',
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash)]
pub struct Zeile {
    pub felder: [Feld; SpielBrett::SIZE / 2],
}

impl Zeile {
    pub fn from_felder(felder: [Feld; SpielBrett::SIZE / 2]) -> Self {
        Self { felder }
    }

    pub fn get(&self, pos: Position) -> Feld {
        if !pos.valid() {
            panic!("invalid position");
        }

        self.felder[pos.spalte / 2]
    }

    pub fn set(&mut self, pos: Position, feld: Feld) {
        if !pos.valid() {
            panic!("invalid position");
        }

        self.felder[pos.spalte / 2] = feld
    }

    pub fn append_to_string(&self, zeile: usize, result: &mut String) {
        for spalte in 0..SpielBrett::SIZE {
            let position = Position { spalte, zeile };
            if position.valid() {
                result.push(self.get(position).to_character());
            } else {
                result.push(' ');
            }
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct SpielBrett {
    pub zeilen: [Zeile; Self::SIZE],
}

impl SpielBrett {
    pub const SIZE: usize = 8;

    pub fn get(&self, pos: Position) -> Feld {
        if !pos.valid() {
            panic!("invalid position");
        }

        self.zeilen[pos.zeile].get(pos)
    }

    pub fn set(&mut self, pos: Position, feld: Feld) {
        if !pos.valid() {
            panic!("invalid position");
        }

        self.zeilen[pos.zeile].set(pos, feld)
    }

    pub fn has_won(&self, winner: Spieler) -> bool {
        for zeile in 0..SpielBrett::SIZE {
            for spalte in 0..SpielBrett::SIZE {
                if let Feld::Stein(player) | Feld::Dame(player) =
                    self.get(Position { zeile, spalte })
                {
                    if player == !winner {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (zeile_index, zeile) in self.zeilen.into_iter().enumerate() {
            zeile.append_to_string(zeile_index, &mut result);
            if zeile_index != Self::SIZE - 1 {
                result.push('\n');
            }
        }
        result
    }
}

impl Debug for SpielBrett {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match f.write_str(&self.to_string()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
