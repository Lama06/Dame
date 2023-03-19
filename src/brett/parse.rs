use super::{Feld, Position, SpielBrett, Spieler, Zeile};

impl Feld {
    fn parse(character: char) -> Option<Self> {
        Some(match character {
            '_' => Self::Leer,
            'm' => Self::Stein(Spieler::Mensch),
            'M' => Self::Dame(Spieler::Mensch),
            'c' => Self::Stein(Spieler::Computer),
            'C' => Self::Dame(Spieler::Computer),
            _ => return None,
        })
    }
}

impl Zeile {
    pub fn parse(zeile: usize, text: &str) -> Option<Self> {
        if text.chars().count() != SpielBrett::SIZE {
            return None;
        }

        let mut result = Self::default();

        for (spalte, character) in text.chars().enumerate() {
            let position = Position { spalte, zeile };

            if !position.valid() {
                if character != ' ' {
                    return None;
                }
                continue;
            }

            result.set(
                position,
                match Feld::parse(character) {
                    Some(feld) => feld,
                    None => return None,
                },
            );
        }

        Some(result)
    }
}

impl SpielBrett {
    pub fn parse(text: &str) -> Option<Self> {
        Some(Self {
            zeilen: {
                if text.lines().count() != Self::SIZE {
                    return None;
                }

                let mut zeilen = [Zeile::default(); Self::SIZE];

                for (zeile_index, zeile_text) in text.lines().enumerate() {
                    let zeile = match Zeile::parse(zeile_index, zeile_text) {
                        Some(zeile) => zeile,
                        None => return None,
                    };

                    zeilen[zeile_index] = zeile;
                }

                zeilen
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brett_parse_1() {
        let brett = SpielBrett::parse(concat!(
            "m m m m \n",
            " m m m m\n",
            "m m m m \n",
            " _ _ _ _\n",
            "_ _ _ _ \n",
            " c c c c\n",
            "c c c c \n",
            " c c c c",
        ))
        .unwrap();

        assert_eq!(
            brett.zeilen[0],
            Zeile::from_felder([
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch)
            ])
        );

        assert_eq!(
            brett.zeilen[1],
            Zeile::from_felder([
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch)
            ])
        );

        assert_eq!(
            brett.zeilen[2],
            Zeile::from_felder([
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch),
                Feld::Stein(Spieler::Mensch)
            ])
        );

        assert_eq!(
            brett.zeilen[3],
            Zeile::from_felder([Feld::Leer, Feld::Leer, Feld::Leer, Feld::Leer,])
        );

        assert_eq!(
            brett.zeilen[4],
            Zeile::from_felder([Feld::Leer, Feld::Leer, Feld::Leer, Feld::Leer,])
        );

        assert_eq!(
            brett.zeilen[5],
            Zeile::from_felder([
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer)
            ])
        );

        assert_eq!(
            brett.zeilen[6],
            Zeile::from_felder([
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer)
            ])
        );

        assert_eq!(
            brett.zeilen[7],
            Zeile::from_felder([
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer),
                Feld::Stein(Spieler::Computer)
            ])
        );
    }

    #[test]
    fn test_brett_parse_2() {
        let brett = SpielBrett::parse(concat!(
            "C c _ m \n",
            " _ M c C\n",
            "_ _ _ _ \n",
            " _ _ _ _\n",
            "_ _ _ _ \n",
            " _ _ _ _\n",
            "_ _ _ _ \n",
            " _ _ _ _",
        ))
        .unwrap();

        assert_eq!(
            brett.zeilen[0],
            Zeile::from_felder([
                Feld::Dame(Spieler::Computer),
                Feld::Stein(Spieler::Computer),
                Feld::Leer,
                Feld::Stein(Spieler::Mensch)
            ])
        );

        assert_eq!(
            brett.zeilen[1],
            Zeile::from_felder([
                Feld::Leer,
                Feld::Dame(Spieler::Mensch),
                Feld::Stein(Spieler::Computer),
                Feld::Dame(Spieler::Computer)
            ])
        );
    }
}
