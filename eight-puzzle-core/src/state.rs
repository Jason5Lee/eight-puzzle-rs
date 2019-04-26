use crate::direction::Direction;
use std::cmp::Eq;
use std::hash::Hash;

pub const NUMBER_OF_STATES: u32 = 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2;

pub trait State: Eq + Hash + Clone
where
    Self: Sized,
{
    /// Returns the board as an array of length 9.
    fn to_array(&self) -> [u8; 9];

    /// Returns the `State` from the board.
    /// If the array is illegal, i.e. not a permutation of 0..9, the result is undefined.
    fn from_array(array: &[u8; 9]) -> Self;

    /// Returns the hash of the states.
    /// It should be in range of 0..NUMBER_OF_STATES.
    fn hash(&self) -> u32;

    fn neighbors(&self) -> Vec<(Direction, Self)> {
        let mut result = Vec::new();
        let mut arr: [u8; 9] = self.to_array();
        for i in 0..9 {
            if arr[i] == 0 {
                let mut add_state = |pre: bool, offset: isize, direction: Direction| {
                    if pre {
                        let new_i = (i as isize + offset) as usize;
                        arr[i] = arr[new_i];
                        arr[new_i] = 0;
                        result.push((direction, Self::from_array(&arr)));
                        arr[new_i] = arr[i];
                    }
                };

                add_state(i >= 3, -3, Direction::Down);
                add_state(i < 6, 3, Direction::Up);
                add_state(i % 3 != 0, -1, Direction::Right);
                add_state(i % 3 != 2, 1, Direction::Left);
            }
        }
        result
    }

    fn move_blank<I: Iterator<Item = usize>>(&self, indices: I, offset: isize) -> Option<Self> {
        let mut arr = self.to_array();
        for i in indices {
            if arr[i] == 0 {
                let new_i = (i as isize + offset) as usize;
                arr[i] = arr[new_i];
                arr[new_i] = 0;

                return Some(Self::from_array(&arr));
            }
        }
        None
    }

    fn up(&self) -> Option<Self> {
        self.move_blank(3..9, -3)
    }

    fn down(&self) -> Option<Self> {
        self.move_blank(0..6, 3)
    }

    fn left(&self) -> Option<Self> {
        self.move_blank((1..9).filter(|x| x % 3 != 0), -1)
    }

    fn right(&self) -> Option<Self> {
        self.move_blank((0..8).filter(|x| x % 3 != 2), 1)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct HashState {
    hash: u32,
}

impl State for HashState {
    fn to_array(&self) -> [u8; 9] {
        let mut result = [0u8; 9];
        {
            let mut rest_hash = self.hash;
            for i in 1u32..=9u32 {
                result[(9 - i) as usize] = (rest_hash % i) as u8;
                rest_hash /= i;
            }
        }
        {
            let mut appeared = [false; 9];
            for ele in &mut result {
                let mut i = 0;
                while i <= *ele {
                    if appeared[i as usize] {
                        *ele += 1
                    }
                    i += 1
                }
                appeared[*ele as usize] = true;
            }
        }

        result
    }

    fn from_array(arr: &[u8; 9]) -> HashState {
        let mut hash = 0u32;
        for i in 0usize..9usize {
            hash = hash * (9 - i as u32) + arr[i] as u32
                - arr.iter().take(i as usize).filter(|&&x| x < arr[i]).count() as u32;
        }

        HashState { hash }
    }

    fn hash(&self) -> u32 {
        self.hash
    }
}
