use crate::direction::Direction;
use crate::state::State;
use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Hinter<S: State> {
    hint_map: HashMap<S, Direction>,
}

impl<S: State> Hinter<S> {
    pub fn new(goal: S) -> Self {
        let mut hint_map = HashMap::new();

        let mut q = VecDeque::new();
        q.push_back(goal);
        while let Some(f) = q.pop_front() {
            for (direction, neighbor) in f.neighbors().into_iter() {
                hint_map.entry(neighbor.clone()).or_insert_with(|| {
                    q.push_back(neighbor);
                    direction
                });
            }
        }
        Hinter { hint_map }
    }

    pub fn hint(&self, s: &S) -> Option<&Direction> {
        self.hint_map.get(s)
    }

    pub fn random_state(&self) -> Option<&S> {
        self.hint_map.keys().choose(&mut rand::thread_rng())
    }
}
