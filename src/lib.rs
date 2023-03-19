use brett::{SpielBrett, Spieler};

pub mod brett;

fn main() {
    SpielBrett::get_possible_moves(&SpielBrett::default(), Spieler::Mensch);
}
