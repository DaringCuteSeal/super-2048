use std::fmt::Display;

use rand::{RngExt, rng, rngs::ThreadRng, seq::IteratorRandom};

// pls above 1
const GRID_WIDTH: usize = 4;

// purely for printing
const TILE_WIDTH: usize = 5;

pub struct GameData {
    numbers_data: [[u32; GRID_WIDTH]; GRID_WIDTH],
    rng: ThreadRng,
}

impl Display for GameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for r in self.numbers_data {
            for c in r {
                if c == 0 {
                    out.push_str("    . ");
                } else {
                    out.push_str(&format!("{:5} ", c));
                }
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

impl GameData {
    pub fn get_inner(&self) -> &[[u32; GRID_WIDTH]; GRID_WIDTH] {
        return &self.numbers_data;
    }

    pub fn new_with(data: [[u32; GRID_WIDTH]; GRID_WIDTH]) -> Self {
        return Self {
            numbers_data: data,
            rng: rng(),
        };
    }

    pub fn new() -> Self {
        return Self {
            numbers_data: [[0; GRID_WIDTH]; GRID_WIDTH],
            rng: rng(),
        };
    }

    // lazy to optimize honestly but its fine
    // not my main point of interest anyway
    pub fn insert_random(&mut self) -> bool {
        let mut empty_tiles = Vec::new();
        for r in 0..GRID_WIDTH {
            for c in 0..GRID_WIDTH {
                if self.numbers_data[r][c] == 0 {
                    empty_tiles.push((r, c));
                }
            }
        }
        if let Some((r, c)) = empty_tiles.iter().choose(&mut self.rng) {
            if self.rng.random_range(1..=10) == 10 {
                self.numbers_data[*r][*c] = 4;
            } else {
                self.numbers_data[*r][*c] = 2;
            }
            return true;
        }
        return false;
    }

    pub fn has_legal_moves(&self) -> bool {
        for r in 0..GRID_WIDTH {
            for c in 0..GRID_WIDTH {
                if self.numbers_data[r][c] == 0 {
                    return true;
                }
                if c != 0 && self.numbers_data[r][c] == self.numbers_data[r][c - 1] {
                    return true;
                }
            }
        }

        for c in 0..GRID_WIDTH {
            for r in 0..GRID_WIDTH {
                if self.numbers_data[r][c] == 0 {
                    return true;
                }
                if r != 0 && self.numbers_data[r][c] == self.numbers_data[r - 1][c] {
                    return true;
                }
            }
        }
        return false;
    }

    // this kool algorithm is an O(N) time complexity, O(1) space complexity merging algorithm
    // that instead of uses a stack, directly writes the resulting state
    // to the input array itself.
    //
    // edit: i mean O(N) for one row/column so technically O(N^2) for this game
    pub fn gravity_down(&mut self) {
        for c in 0..GRID_WIDTH {
            let mut start = GRID_WIDTH - 1;

            while start != 0 && self.numbers_data[start][c] == 0 {
                start -= 1;
            }

            let tmp = self.numbers_data[start][c];
            self.numbers_data[start][c] = self.numbers_data[GRID_WIDTH - 1][c];
            self.numbers_data[GRID_WIDTH - 1][c] = tmp;

            let mut write_ptr = GRID_WIDTH - 1;
            if start == 0 {
                continue;
            }
            for i in (0..=(start - 1)).rev() {
                if self.numbers_data[i][c] == 0 {
                    continue;
                }

                if self.numbers_data[i][c] == self.numbers_data[write_ptr][c] {
                    self.numbers_data[write_ptr][c] *= 2;

                    // merge backwards if possible
                    while write_ptr + 1 < GRID_WIDTH
                        && self.numbers_data[write_ptr][c] == self.numbers_data[write_ptr + 1][c]
                    {
                        self.numbers_data[write_ptr + 1][c] *= 2;
                        write_ptr += 1;
                    }
                } else {
                    write_ptr -= 1;
                    self.numbers_data[write_ptr][c] = self.numbers_data[i][c];
                }
            }
            for i in 0..write_ptr {
                self.numbers_data[i][c] = 0;
            }
        }
    }

    pub fn gravity_right(&mut self) {
        for r in 0..GRID_WIDTH {
            let mut start = GRID_WIDTH - 1;

            while start != 0 && self.numbers_data[r][start] == 0 {
                start -= 1;
            }

            let tmp = self.numbers_data[r][start];
            self.numbers_data[r][start] = self.numbers_data[r][GRID_WIDTH - 1];
            self.numbers_data[r][GRID_WIDTH - 1] = tmp;

            let mut write_ptr = GRID_WIDTH - 1;
            if start == 0 {
                continue;
            }
            for i in (0..=(start - 1)).rev() {
                if self.numbers_data[r][i] == 0 {
                    continue;
                }

                if self.numbers_data[r][i] == self.numbers_data[r][write_ptr] {
                    self.numbers_data[r][write_ptr] *= 2;

                    // merge backwards if possible
                    while write_ptr + 1 < GRID_WIDTH
                        && self.numbers_data[r][write_ptr] == self.numbers_data[r][write_ptr + 1]
                    {
                        self.numbers_data[r][write_ptr + 1] *= 2;
                        write_ptr += 1;
                    }
                } else {
                    write_ptr -= 1;
                    self.numbers_data[r][write_ptr] = self.numbers_data[r][i];
                }
            }
            for i in 0..write_ptr {
                self.numbers_data[r][i] = 0;
            }
        }
    }

    pub fn gravity_up(&mut self) {
        for c in 0..GRID_WIDTH {
            let mut start = 0;

            while start + 1 < GRID_WIDTH && self.numbers_data[start][c] == 0 {
                start += 1;
            }

            let tmp = self.numbers_data[start][c];
            self.numbers_data[start][c] = self.numbers_data[0][c];
            self.numbers_data[0][c] = tmp;

            let mut write_ptr = 0;
            if start == GRID_WIDTH - 1 {
                continue;
            }
            for i in start + 1..GRID_WIDTH {
                if self.numbers_data[i][c] == 0 {
                    continue;
                }

                if self.numbers_data[i][c] == self.numbers_data[write_ptr][c] {
                    self.numbers_data[write_ptr][c] *= 2;

                    // merge backwards if possible
                    while write_ptr >= 1
                        && self.numbers_data[write_ptr][c] == self.numbers_data[write_ptr - 1][c]
                    {
                        self.numbers_data[write_ptr - 1][c] *= 2;
                        write_ptr -= 1;
                    }
                } else {
                    write_ptr += 1;
                    self.numbers_data[write_ptr][c] = self.numbers_data[i][c];
                }
            }
            for i in write_ptr + 1..GRID_WIDTH {
                self.numbers_data[i][c] = 0;
            }
        }
    }

    pub fn gravity_left(&mut self) {
        for r in 0..GRID_WIDTH {
            let mut start = 0;

            while start + 1 < GRID_WIDTH && self.numbers_data[r][start] == 0 {
                start += 1;
            }

            let tmp = self.numbers_data[r][start];
            self.numbers_data[r][start] = self.numbers_data[r][0];
            self.numbers_data[r][0] = tmp;

            let mut write_ptr = 0;
            if start == GRID_WIDTH - 1 {
                continue;
            }
            for i in start + 1..GRID_WIDTH {
                if self.numbers_data[r][i] == 0 {
                    continue;
                }

                if self.numbers_data[r][i] == self.numbers_data[r][write_ptr] {
                    self.numbers_data[r][write_ptr] *= 2;

                    // merge backwards if possible
                    while write_ptr >= 1
                        && self.numbers_data[r][write_ptr] == self.numbers_data[r][write_ptr - 1]
                    {
                        self.numbers_data[r][write_ptr - 1] *= 2;
                        write_ptr -= 1;
                    }
                } else {
                    write_ptr += 1;
                    self.numbers_data[r][write_ptr] = self.numbers_data[r][i];
                }
            }
            for i in write_ptr + 1..GRID_WIDTH {
                self.numbers_data[r][i] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_data::GameData;

    #[test]
    fn test_print() {
        let data = GameData::new();
        assert_eq!(
            data.to_string(),
            "    .     .     .     . \n    .     .     .     . \n    .     .     .     . \n    .     .     .     . \n"
        );
    }

    #[test]
    fn test_gravity_down() {
        #[rustfmt::skip]
        let tiles = [
            [0, 2, 2, 4],
            [4, 0, 8, 4],
            [0, 0, 8, 0],
            [2, 2, 16, 2]
        ];
        #[rustfmt::skip]
        let correct = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [4, 0, 2, 8],
            [2, 4, 32, 2]
        ];

        let mut data = GameData::new_with(tiles);
        data.gravity_down();
        print!("{}", data);
        assert_eq!(&correct, data.get_inner());
    }

    #[test]
    fn test_gravity_up() {
        #[rustfmt::skip]
        let tiles = [
            [0, 2, 2, 4],
            [4, 2, 8, 4],
            [0, 4, 8, 0],
            [2, 8, 16, 2]
        ];
        #[rustfmt::skip]
        let correct = [
            [4, 16, 2, 8],
            [2, 0, 32, 2],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ];

        let mut data = GameData::new_with(tiles);
        data.gravity_up();
        print!("{}", data);
        assert_eq!(&correct, data.get_inner());
    }

    #[test]
    fn test_gravity_left() {
        #[rustfmt::skip]
        let tiles = [
            [0, 2, 2, 4],
            [4, 2, 8, 4],
            [0, 4, 8, 0],
            [2, 8, 8, 16]
        ];
        #[rustfmt::skip]
        let correct = [
            [8, 0, 0, 0],
            [4, 2, 8, 4],
            [4, 8, 0, 0],
            [2, 32, 0, 0]
        ];

        let mut data = GameData::new_with(tiles);
        data.gravity_left();
        print!("{}", data);
        assert_eq!(&correct, data.get_inner());
    }

    #[test]
    fn test_gravity_right() {
        #[rustfmt::skip]
        let tiles = [
            [2, 2, 2, 4],
            [4, 2, 8, 4],
            [0, 0, 8, 0],
            [2, 8, 8, 16]
        ];
        #[rustfmt::skip]
        let correct = [
            [0, 0, 2, 8],
            [4, 2, 8, 4],
            [0, 0, 0, 8],
            [0, 0, 2, 32]
        ];

        let mut data = GameData::new_with(tiles);
        data.gravity_right();
        print!("{}", data);
        assert_eq!(&correct, data.get_inner());
    }

    #[test]
    fn no_legal_moves() {
        #[rustfmt::skip]
        let tiles = [
            [1, 2, 3, 4],
            [4, 3, 2, 1],
            [1, 2, 3, 4],
            [4, 3, 2, 1]
        ];
        let data = GameData::new_with(tiles);
        assert!(!data.has_legal_moves());
    }

    #[test]
    fn has_legal_moves() {
        #[rustfmt::skip]
        let tiles = [
            [2, 2, 3, 4],
            [4, 3, 2, 1],
            [1, 2, 3, 4],
            [4, 3, 2, 1]
        ];
        let data = GameData::new_with(tiles);
        assert!(data.has_legal_moves());
    }

    #[test]
    fn has_legal_moves_2() {
        #[rustfmt::skip]
        let tiles = [
            [0, 2, 3, 4],
            [4, 3, 2, 1],
            [1, 2, 3, 4],
            [4, 3, 2, 1]
        ];
        let data = GameData::new_with(tiles);
        assert!(data.has_legal_moves());
    }

    #[test]
    fn has_legal_moves_3() {
        #[rustfmt::skip]
        let tiles = [
            [0, 2, 3, 0],
            [4, 3, 2, 0],
            [1, 2, 3, 0],
            [4, 3, 2, 0]
        ];
        let data = GameData::new_with(tiles);
        assert!(data.has_legal_moves());
    }

    #[test]
    fn has_legal_moves_4() {
        #[rustfmt::skip]
        let tiles = [
            [4, 2, 32, 2],
            [2, 8, 2, 4],
            [4, 64, 8, 32],
            [16, 8, 2, 0]
        ];
        let data = GameData::new_with(tiles);
        assert!(data.has_legal_moves());
    }
}
