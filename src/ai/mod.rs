use crate::brett::{SpielBrett, Spieler};

mod statistik;

impl SpielBrett {
    pub fn calculate_best_next_move(&self, depth: u32) -> Self {
        if depth == 0 {
            return self.clone();
        }
        let possible_moves = self.get_possible_moves(Spieler::Computer);
        if possible_moves.len() == 0 {
            return self.clone();
        }
        let antworten_des_gegners = possible_moves.into_iter().map(|brett| {
            let antwort_des_gegners = brett.simulate_human();
            (brett, antwort_des_gegners)
        });
        let antworten_auf_gegner = antworten_des_gegners
            .map(|(original, brett)| (original, brett.calculate_best_next_move(depth - 1)));
        let bestes_antowrt = antworten_auf_gegner.max_by(|(_, brett1), (_, brett2)| {
            brett1
                .get_statistik()
                .to_number(Spieler::Computer)
                .cmp(&brett2.get_statistik().to_number(Spieler::Computer))
        });
        bestes_antowrt.unwrap().0
    }

    /// Macht den Zug für den Menschen, bei dem das beste unmittelbare Ergebnis rauskommt
    fn simulate_human(&self) -> Self {
        let possible_moves = self.get_possible_moves(Spieler::Mensch);
        if possible_moves.is_empty() {
            return self.clone();
        }
        let best_move = possible_moves.into_iter().max_by(|move1, move2| {
            move1
                .get_statistik()
                .to_number(Spieler::Mensch)
                .cmp(&move2.get_statistik().to_number(Spieler::Mensch))
        });
        best_move.unwrap() // Kann nie None sein, weil possible_moves nicht empty ist
    }
}
