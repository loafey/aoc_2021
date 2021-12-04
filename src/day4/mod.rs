use core::{fmt, num};

use crate::load;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    paper: Vec<Vec<Num>>,
}
impl Board {
    fn load_input(s: String) -> (Vec<Board>, Vec<i32>) {
        let mut s = s.split("\n\n");
        let numbers = s
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let boards = s
            .map(|b_str| {
                Board::new(
                    &b_str
                        .split('\n')
                        .map(|r| r.split(' '))
                        .flatten()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        (boards, numbers)
    }

    fn new(nums: &[i32]) -> Self {
        let mut paper = Vec::with_capacity(BOARD_SIZE);

        let chunks = nums.chunks(BOARD_SIZE);
        chunks.for_each(|c| {
            let mut r = Vec::with_capacity(BOARD_SIZE);
            c.iter().for_each(|v| r.push(Num::new(*v)));
            paper.push(r);
        });

        Self { paper }
    }

    pub fn add_num(&mut self, n: i32) {
        self.paper.iter_mut().for_each(|r| {
            r.iter_mut().for_each(|v| {
                if v.val == n {
                    v.found = true;
                }
            })
        });
    }

    pub fn won(&self, num: i32) -> Option<i32> {
        let mut won = false;
        // ROW WIN
        for y in &self.paper {
            let mut row_won = true;
            for x in y {
                row_won &= x.found;
            }
            if row_won {
                won = true;
                break;
            }
        }

        // COLLUMN WIN
        for c in 0..self.paper.len() {
            let mut collumn_won = true;
            for y in &self.paper {
                collumn_won &= y[c].found;
            }
            if collumn_won {
                won = true;
                break;
            }
        }

        if won {
            let sum = self
                .paper
                .iter()
                .flatten()
                .filter(|n| !n.found)
                .map(|n| n.val)
                .sum::<i32>();
            Some(sum * num)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Num {
    val: i32,
    found: bool,
}
impl Num {
    fn new(val: i32) -> Self {
        Self { val, found: false }
    }
}

pub fn part1() -> i32 {
    let (mut boards, numbers) = Board::load_input(load("input/day4.txt"));

    let mut ans = 0;
    for n in numbers {
        let mut found_winner = false;
        boards.iter_mut().for_each(|b| {
            b.add_num(n);
            if let Some(s) = b.won(n) {
                ans = s;
                found_winner = true;
            }
        });
        if found_winner {
            break;
        }
    }
    ans
}

pub fn part2() -> i32 {
    let (mut boards, numbers) = Board::load_input(load("input/day4.txt"));

    let mut ans = 0;
    for n in numbers {
        let new_boards = boards
            .clone()
            .into_iter()
            .map(|mut b| {
                b.add_num(n);
                b
            })
            .filter(|b| b.won(n).is_none())
            .collect::<Vec<_>>();
        if !new_boards.is_empty() {
            boards = new_boards;
        } else {
            boards[0].add_num(n);
            if let Some(v) = boards[0].won(n) {
                ans = v;
            }
            break;
        }
    }

    ans
}
