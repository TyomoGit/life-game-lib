use std::collections::VecDeque;

use rand::{thread_rng, Rng};

const PREVS_MAX_LENGTH: usize = 10_usize.pow(4);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board {
    board: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(board: Vec<Vec<bool>>) -> Self {
        Self { board }
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }

    pub fn width(&self) -> usize {
        self.board.first().unwrap().len()
    }

    pub fn board(&self) -> &Vec<Vec<bool>> {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Vec<Vec<bool>> {
        &mut self.board
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.board.get(y)?.get(x).cloned()
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.board[y][x] = value;
    }
}

pub struct Game {
    pub init_board: Board,

    board: Board,
    buffer: Board,

    prevs: VecDeque<Board>,
    prevs_max_length: usize,

    is_torus: bool,
    epochs: usize,
}

impl Game {
    pub fn new(board: Board, is_torus: bool) -> Self {
        Self {
            board: board.clone(),
            buffer: board.clone(),
            is_torus,
            init_board: board.clone(),
            prevs: VecDeque::new(),
            prevs_max_length: PREVS_MAX_LENGTH,
            epochs: 0,
        }
    }

    pub fn new_random(width: usize, height: usize, is_torus: bool) -> Self {
        let mut board = Board::new(vec![Vec::with_capacity(width); height]);
        let mut rng = thread_rng();

        for row in board.board_mut() {
            for _ in 0..width {
                row.push(rng.gen_bool(0.5));
            }
        }

        Self {
            init_board: board.clone(),
            board: board.clone(),
            buffer: board.clone(),
            prevs: VecDeque::new(),
            prevs_max_length: PREVS_MAX_LENGTH,
            is_torus,
            epochs: 0,
        }
    }

    pub fn height(&self) -> usize {
        self.board.height()
    }

    pub fn width(&self) -> usize {
        self.board.width()
    }

    pub fn board(&self) -> &Vec<Vec<bool>> {
        self.board.board()
    }

    pub fn epochs(&self) -> usize {
        self.epochs
    }

    fn is_dead(&self) -> bool {
        for elem in &self.prevs {
            if elem == &self.board {
                return true;
            }
        }

        false
    }

    pub fn step_until_dead(&mut self) {
        while !self.is_dead() {
            self.prevs.push_back(self.board.clone());
            if self.prevs.len() > self.prevs_max_length {
                self.prevs.pop_front();
            }

            self.step();
        }
    }

    pub fn step(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let neighbor_count = self.count_neighbors(x, y);

                if self.buffer.get(x, y).unwrap() {
                    if !(neighbor_count == 2 || neighbor_count == 3) {
                        self.buffer.set(x, y, false);
                    }
                } else if neighbor_count == 3 {
                    self.buffer.set(x, y, true);
                }
            }
        }
        self.board = self.buffer.clone();
        self.epochs += 1;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut count: usize = 0;

        for direction in DIRECTIONS {
            let x = x as i32 + direction.0;
            let y = y as i32 + direction.1;

            if self.check_within_range(x, y) {
                if self.board.get(x as usize, y as usize).unwrap() {
                    count += 1;
                }
            } else if self.is_torus {
                let x = (x.rem_euclid(self.width() as i32))as usize;
                let y = (y.rem_euclid(self.height() as i32))as usize;

                if self.board.get(x, y).unwrap() {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn check_within_range(&self, x: i32, y: i32) -> bool {
        0 <= x && (x as usize) < self.width() && 0 <= y && (y as usize) < self.height()
    }
}
