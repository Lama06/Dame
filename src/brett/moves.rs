use std::collections::HashSet;

use super::{Feld, Position, RichtungHorizontal, RichtungVertikal, SpielBrett, Spieler};

impl SpielBrett {
    fn append_stein_schlagen_moves(
        &self,
        position: Position,
        backwards: bool,
        moves: &mut HashSet<Self>,
    ) {
        if !position.valid() {
            return;
        }
        let spieler = match self.get(position) {
            Feld::Stein(spieler) => spieler,
            Feld::Leer | Feld::Dame(_) => return,
        };

        let richtungen_v_move_direction = [spieler.move_direction()];
        let richtungen_v_beide = [RichtungVertikal::Oben, RichtungVertikal::Unten];

        let richtungen_v: &[RichtungVertikal] = match backwards {
            false => &richtungen_v_move_direction,
            true => &richtungen_v_beide,
        };

        for richtung_v in richtungen_v {
            for richtung_h in [RichtungHorizontal::Links, RichtungHorizontal::Rechts] {
                let schlagen_position = Position {
                    spalte: match position.spalte as isize + richtung_h.offset() {
                        spalte @ 0.. => spalte as usize,
                        _ => continue,
                    },
                    zeile: match position.zeile as isize + richtung_v.offset() {
                        zeile @ 0.. => zeile as usize,
                        _ => continue,
                    },
                };
                if !schlagen_position.valid() {
                    continue;
                }
                if let Feld::Stein(schlagen_spieler) | Feld::Dame(schlagen_spieler) =
                    self.get(schlagen_position)
                {
                    if schlagen_spieler == spieler {
                        continue;
                    }
                } else {
                    continue;
                }

                let neue_position = Position {
                    spalte: match position.spalte as isize + richtung_h.offset() * 2 {
                        spalte @ 0.. => spalte as usize,
                        _ => continue,
                    },
                    zeile: match position.zeile as isize + richtung_v.offset() * 2 {
                        zeile @ 0.. => zeile as usize,
                        _ => continue,
                    },
                };
                if !neue_position.valid() {
                    continue;
                }
                if !matches!(self.get(neue_position), Feld::Leer) {
                    continue;
                }

                let mut neues_brett = self.clone();
                neues_brett.set(position, Feld::Leer);
                neues_brett.set(schlagen_position, Feld::Leer);
                neues_brett.set(
                    neue_position,
                    if neue_position.zeile == spieler.dame_zeile() {
                        Feld::Dame(spieler)
                    } else {
                        Feld::Stein(spieler)
                    },
                );

                let mut following_moves = HashSet::new();
                neues_brett.append_stein_schlagen_moves(neue_position, true, &mut following_moves);
                if following_moves.len() == 0 {
                    moves.insert(neues_brett);
                } else {
                    for following_move in following_moves {
                        moves.insert(following_move);
                    }
                }
            }
        }
    }

    fn append_all_stein_schlagen_moves(&self, spieler: Spieler, moves: &mut HashSet<Self>) {
        for spalte in 0..SpielBrett::SIZE {
            for zeile in 0..SpielBrett::SIZE {
                let position = Position { spalte, zeile };
                if !position.valid() {
                    continue;
                }

                if let Feld::Stein(stein_spieler) = self.get(position) {
                    if stein_spieler != spieler {
                        continue;
                    }
                } else {
                    continue;
                }

                self.append_stein_schlagen_moves(position, false, moves);
            }
        }
    }

    fn append_all_stein_moves(&self, spieler: Spieler, moves: &mut HashSet<Self>) {
        for spalte in 0..SpielBrett::SIZE {
            for zeile in 0..SpielBrett::SIZE {
                let position = Position { spalte, zeile };
                if !position.valid() {
                    continue;
                }

                if let Feld::Stein(stein_spieler) = self.get(position) {
                    if stein_spieler != spieler {
                        continue;
                    }
                } else {
                    continue;
                }

                for richtung in [RichtungHorizontal::Links, RichtungHorizontal::Rechts] {
                    let neue_position = Position {
                        spalte: match position.spalte as isize + richtung.offset() {
                            spalte @ 0.. => spalte as usize,
                            _ => continue,
                        },
                        zeile: match position.zeile as isize + spieler.move_direction().offset() {
                            zeile @ 0.. => zeile as usize,
                            _ => continue,
                        },
                    };
                    if !neue_position.valid() {
                        continue;
                    }
                    if !matches!(self.get(neue_position), Feld::Leer) {
                        continue;
                    }

                    let mut neues_brett = self.clone();
                    neues_brett.set(position, Feld::Leer);
                    neues_brett.set(
                        neue_position,
                        if neue_position.zeile == spieler.dame_zeile() {
                            Feld::Dame(spieler)
                        } else {
                            Feld::Stein(spieler)
                        },
                    );
                    moves.insert(neues_brett);
                }
            }
        }
    }

    pub fn get_possible_moves(&self, spieler: Spieler) -> HashSet<Self> {
        let mut moves = HashSet::new();
        self.append_all_stein_schlagen_moves(spieler, &mut moves);
        println!("{}", moves.len());
        if !moves.is_empty() {
            return moves;
        }
        self.append_all_stein_moves(spieler, &mut moves);
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_moves {
        ($($input_zeile: literal)* possible moves for $spieler:path: $($($output_zeile: literal)*)or*) => {
            let mut brett_text = String::new();
            $({
                brett_text.push_str($input_zeile);
                brett_text.push('\n');
            })*
            let brett = SpielBrett::parse(&brett_text).unwrap();
            
            let mut possible_moves = HashSet::new();
            $({
                let mut brett_text = String::new();
                $({
                    brett_text.push_str($output_zeile);
                    brett_text.push('\n');
                })*
                let brett = SpielBrett::parse(&brett_text).unwrap();
                possible_moves.insert(brett);
            })*

            assert_eq!(brett.get_possible_moves($spieler), possible_moves);
        };
    }

    #[test]
    fn test_move_stein() {
        test_moves!(
            "_ _ m _ "
            " _ _ M _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " c _ _ _"

            possible moves for Spieler::Computer:
            "_ _ m _ "
            " _ _ M _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "c _ _ _ "
            " _ _ _ _"
            or
            "_ _ m _ "
            " _ _ M _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ c _ _ "
            " _ _ _ _"
        );

        // Am Rand
        test_moves!(
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "c _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"

            possible moves for Spieler::Computer:
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " c _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
        );

        // Zur Dame konvertieren
        test_moves!(
            "_ _ _ _ "
            " _ c _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"

            possible moves for Spieler::Computer:
            "_ C _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            or
            "_ _ C _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
        );

        // Mehrere Figuren
        test_moves!(
            "m _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ m _ _ "
            " _ _ _ c"
            
            possible moves for Spieler::Mensch:
            "_ _ _ _ "
            " m _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ m _ _ "
            " _ _ _ c"
            or
            "m _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " M _ _ c"
            or
            "m _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ M _ c"
        );
    }

    #[test]
    fn test_stein_schlagen() {
        test_moves!(
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ m _ _ "
            " c _ _ _"

            possible moves for Spieler::Computer:
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ c _ _"
            "_ _ _ _ "
            " _ _ _ _"
        );

        // Zur Dame konvertieren
        test_moves!(
            "_ _ _ _ "
            " _ m _ _"
            "_ c _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"

            possible moves for Spieler::Computer:
            "_ _ C _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
        );

        // Mehrere Möglichkeiten
        test_moves!(
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ M M _ "
            " c c c c"

            possible moves for Spieler::Computer:
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ c _ _"
            "_ _ M _ "
            " _ c c c"
            or
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " c _ _ _"
            "_ _ M _ "
            " c _ c c"
            or
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ c _"
            "_ M _ _ "
            " c _ c c"
            or
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ c _ _"
            "_ M _ _ "
            " c c _ c"
        );

        // Nacheinander mehrere Steine schlagen
        test_moves!(
            "_ m _ _ "
            " _ c _ _"
            "_ _ _ _ "
            " _ _ c _"
            "_ _ _ _ "
            " _ _ c _"
            "_ _ _ _ "
            " _ c _ _"

            possible moves for Spieler::Mensch:
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ m _ "
            " _ c _ _"
        );

        // Nacheinander mehrere Steine schlagen rückwärts
        test_moves!(
            "_ m _ _ "
            " _ c c _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"

            possible moves for Spieler::Mensch:
            "_ _ _ m "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
        );

        // Mehrere Möglichkeiten + mehrere Steine gleichzeitig schlagen
        test_moves!(
            "_ m _ _ "
            " c c c _"
            "_ _ _ _ "
            " c _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"

            possible moves for Spieler::Mensch:
            "_ _ _ _ "
            " _ c c _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ m _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            or
            "_ _ _ m "
            " c _ _ _"
            "_ _ _ _ "
            " c _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
            "_ _ _ _ "
            " _ _ _ _"
        );
    }
}
