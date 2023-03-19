use std::cmp::Ordering;

use crate::brett::{Feld, Position, SpielBrett, Spieler};

#[derive(Clone, Copy, Debug)]
pub struct SpielerStatistik {
    pub steine: u32,
    pub damen: u32,
}

impl SpielerStatistik {
    fn to_number(self) -> i32 {
        match self.steine as i32 + self.damen as i32 * 3 {
            0 => i32::MIN,
            number => number,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Statistik {
    pub computer: SpielerStatistik,
    pub mensch: SpielerStatistik,
}

impl Statistik {
    pub fn to_number(self, perspektive: Spieler) -> i32 {
        match perspektive {
            Spieler::Mensch => self
                .mensch
                .to_number()
                .saturating_sub(self.computer.to_number()),
            Spieler::Computer => self
                .computer
                .to_number()
                .saturating_sub(self.mensch.to_number()),
        }
    }
}

impl SpielBrett {
    fn count_felder(&self, feld: Feld) -> u32 {
        let mut count = 0;
        for zeile in 0..Self::SIZE {
            for spalte in 0..Self::SIZE {
                let position = Position { zeile, spalte };
                if !position.valid() {
                    continue;
                }
                if self.get(position) == feld {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn get_statistik(&self) -> Statistik {
        Statistik {
            computer: SpielerStatistik {
                steine: self.count_felder(Feld::Stein(Spieler::Computer)),
                damen: self.count_felder(Feld::Dame(Spieler::Computer)),
            },
            mensch: SpielerStatistik {
                steine: self.count_felder(Feld::Stein(Spieler::Mensch)),
                damen: self.count_felder(Feld::Dame(Spieler::Mensch)),
            },
        }
    }
}
