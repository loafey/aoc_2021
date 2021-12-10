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

fn delta(p1: &Pos, p2: &Pos) -> (i32, i32) {
    (
        match p1.x.cmp(&p2.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        },
        match p1.y.cmp(&p2.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        },
    )
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

    let largest_x = largest(|(p1, p2)| [p1.x, p2.x]);
    let largest_y = largest(|(p1, p2)| [p1.y, p2.y]);

    let mut matrix = vec![0; ((largest_x + 1) * (largest_y + 1)) as usize];

    s.iter().for_each(|(p1, p2)| {
        let (x_mod, y_mod) = delta(p1, p2);

        let (mut x, mut y) = (p1.x, p1.y);
        while x != p2.x + x_mod || y != p2.y + y_mod {
            matrix[(y + (x * largest_y)) as usize] += 1;
            x += x_mod;
            y += y_mod;
        }
    });

    matrix.into_iter().filter(|v| *v > 1).count()
}

pub fn part1() -> usize {
    solver("input/day5.txt", |(p1, p2)| p1.is_hoz_ver(p2))
}

pub fn part2() -> usize {
    solver("input/day5.txt", |(p1, p2)| p1.is_valid(p2))
}
