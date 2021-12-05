use std::{cmp::Ordering, path::Path};

use crate::load_to_rows_and_pattern;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_str(s: &str) -> Self {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse::<i32>().unwrap();
        let y = s.next().unwrap().parse::<i32>().unwrap();
        Self::new(x, y)
    }
    fn is_hoz_ver(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y
    }
    fn is_45(&self, other: &Self) -> bool {
        (other.x - self.x).abs() == (other.y - self.y).abs()
    }
    pub fn is_valid(&self, other: &Self) -> bool {
        self.is_hoz_ver(other) || self.is_45(other)
    }
}

fn solver<A>(path: A, filter: fn(&(Pos, Pos)) -> bool) -> usize
where
    A: AsRef<Path> + std::fmt::Debug + Copy,
{
    let s = load_to_rows_and_pattern(path, |c| c == ' ')
        .map(|r| {
            let mut r = r.filter(|s| s != "->");
            let p1 = Pos::from_str(&r.next().unwrap());
            let p2 = Pos::from_str(&r.next().unwrap());
            (p1, p2)
        })
        .filter(filter)
        .collect::<Vec<_>>();

    let largest = |map: fn(&(Pos, Pos)) -> [i32; 2]| {
        s.iter()
            .map(map)
            .flatten()
            .reduce(|a, v| if a >= v { a } else { v })
            .unwrap_or_default()
    };
    let delta = |p1: i32, p2: i32| match p1.cmp(&p2) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    };

    let largest_x = largest(|(p1, p2)| [p1.x, p2.x]);
    let largest_y = largest(|(p1, p2)| [p1.y, p2.y]);

    let mut matrix = vec![vec![0; largest_x as usize + 1]; largest_y as usize + 1];

    s.iter().for_each(|(p1, p2)| {
        let x_mod = delta(p1.x, p2.x);
        let y_mod = delta(p1.y, p2.y);

        let mut x = p1.x;
        let mut y = p1.y;
        while x != p2.x + x_mod || y != p2.y + y_mod {
            matrix[y as usize][x as usize] += 1;
            x += x_mod;
            y += y_mod;
        }
    });

    matrix.iter().flatten().filter(|v| **v > 1).count()
}

pub fn part1() -> usize {
    solver("input/day5.txt", |(p1, p2)| p1.is_hoz_ver(p2))
}

pub fn part2() -> usize {
    solver("input/day5.txt", |(p1, p2)| p1.is_valid(p2))
}
