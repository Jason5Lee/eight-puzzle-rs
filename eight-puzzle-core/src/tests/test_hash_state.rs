use crate::direction::Direction;
use crate::state::HashState;
use crate::state::State;
use crate::state::NUMBER_OF_STATES;
use permutator::Permutation;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_array() {
    [0, 1, 2, 3, 4, 5, 6, 7, 8].permutation().for_each(|v| {
        let v = [v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]];
        let hash_state = HashState::from_array(&v);
        assert!(hash_state.hash() < NUMBER_OF_STATES);
        assert_eq!(v, hash_state.to_array());
    })
}

#[test]
fn test_neighbors() {
    {
        let conner = HashState::from_array(&[3, 1, 0, 2, 4, 6, 5, 7, 8]);
        assert_eq!(
            HashSet::from_iter(
                [
                    (
                        Direction::Up,
                        HashState::from_array(&[3, 1, 6, 2, 4, 0, 5, 7, 8])
                    ),
                    (
                        Direction::Right,
                        HashState::from_array(&[3, 0, 1, 2, 4, 6, 5, 7, 8])
                    )
                ]
                .iter()
                .cloned()
            ) as HashSet<(Direction, HashState)>,
            HashSet::from_iter(conner.neighbors())
        );
    }
    {
        let edge = HashState::from_array(&[1, 4, 6, 0, 5, 2, 8, 3, 7]);
        assert_eq!(
            HashSet::from_iter(
                [
                    (
                        Direction::Down,
                        HashState::from_array(&[0, 4, 6, 1, 5, 2, 8, 3, 7])
                    ),
                    (
                        Direction::Left,
                        HashState::from_array(&[1, 4, 6, 5, 0, 2, 8, 3, 7])
                    ),
                    (
                        Direction::Up,
                        HashState::from_array(&[1, 4, 6, 8, 5, 2, 0, 3, 7])
                    )
                ]
                .iter()
                .cloned()
            ) as HashSet<(Direction, HashState)>,
            HashSet::from_iter(edge.neighbors())
        );
    }
}
