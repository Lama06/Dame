use std::collections::HashSet;

use crate::brett::{SpielBrett, Spieler};

mod statistik;

impl SpielBrett {
    pub fn get_best_move(&self, max_depth: u32) -> SpielBrett {
        type NodeIndex = usize;

        #[derive(Debug)]
        struct Node {
            depth: u32,
            brett: SpielBrett,
            am_zug: Spieler,
            bewertung: Option<i32>,
            best_child: Option<NodeIndex>,
            children: Vec<NodeIndex>,
        }

        let mut nodes: Vec<Node> = Vec::new();
        nodes.push(Node {
            bewertung: None,
            depth: 0,
            brett: self.clone(),
            am_zug: Spieler::Computer,
            children: Vec::new(),
            best_child: None,
        });

        // Mögliche Züge generieren
        for depth in 1..=max_depth {
            for node_index in 0..nodes.len() {
                let node = &nodes[node_index];
                let node_am_zug = node.am_zug;
                if node.depth != depth-1 {
                    continue;
                }

                let possible_moves = match node.brett.get_possible_moves(node.am_zug) {
                    possible_moves if !possible_moves.is_empty() => possible_moves,
                    _ => {
                        let mut possible_moves = HashSet::new();
                        possible_moves.insert(node.brett.clone());
                        possible_moves
                    },
                };

                for child_brett in possible_moves {
                    let child_index = nodes.len();
                    nodes.push(Node {
                        depth,
                        am_zug: !node_am_zug,
                        bewertung: None,
                        brett: child_brett,
                        best_child: None,
                        children: Vec::new(),
                    });
                    let node = &mut nodes[node_index];
                    node.children.push(child_index);
                }
            }
        }

        // Bewertungen der untersten Zeile berechnen
        for node in nodes.iter_mut() {
            if node.depth != max_depth {
                continue;
            }

            node.bewertung = Some(node.brett.get_statistik().to_number(Spieler::Computer));
        }

        // Bewertungen der Zeilen darüber berechnen
        for depth in (0..=max_depth-1).rev() {
            for node_index in 0..nodes.len() {
                let node = &nodes[node_index];
                if node.depth != depth {
                    continue;
                }

                #[derive(Clone, Copy)]
                struct BestChild {
                    index: NodeIndex,
                    bewertung: i32,
                }

                let mut best_child: Option<BestChild> = None;

                for &child_index in &node.children {
                    let child = &nodes[child_index];
                    let child_bewertung = child.bewertung.unwrap();
                    best_child = Some(match best_child {
                        None => BestChild { index: child_index, bewertung: child_bewertung },
                        Some(best_child) => match node.am_zug {
                            Spieler::Computer => if best_child.bewertung > child_bewertung {
                                best_child
                            } else {
                                BestChild { index: child_index, bewertung: child_bewertung }
                            },
                            Spieler::Mensch => if best_child.bewertung < child_bewertung {
                                best_child
                            } else {
                                BestChild { index: child_index, bewertung: child_bewertung }
                            },
                        }
                    });
                }

                let node = &mut nodes[node_index];
                node.bewertung = Some(best_child.unwrap().bewertung);
                node.best_child = Some(best_child.unwrap().index);
            }
        }

        nodes[nodes[0].best_child.unwrap()].brett.clone()
    }
}
