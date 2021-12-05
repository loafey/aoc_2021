use std::ops::Range;

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
    pub fn is_hoz_ver(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y
    }
    pub fn is_45(&self, other: &Self) -> bool {
        ((other.x - self.x).abs() == (other.y - self.y).abs())
    }
    pub fn is_valid(&self, other: &Self) -> bool {
        self.is_hoz_ver(other) || self.is_45(other)
    }

    pub fn create_range(&self, other: &Self) -> (Range<i32>, Range<i32>) {
        (self.x..other.x + 1, self.y..other.y + 1)
    }

    pub fn cas_hoz(&mut self, other: &mut Self) {
        if self.x > other.x {
            std::mem::swap(&mut self.x, &mut other.x);
        } else if self.y > other.y {
            std::mem::swap(&mut self.y, &mut other.y);
        }
    }
}

pub fn part1() -> usize {
    let s = load_to_rows_and_pattern("input/day5.txt", |c| c == ' ');
    let s = s
        .map(|r| {
            let mut r = r.filter(|s| s != "->");
            let mut p1 = Pos::from_str(&r.next().unwrap());
            let mut p2 = Pos::from_str(&r.next().unwrap());
            p1.cas_hoz(&mut p2);
            (p1, p2)
        })
        .filter(|(p1, p2)| p1.is_hoz_ver(p2))
        //.map(|(p1, p2)| p1.to_range(&p2))
        .collect::<Vec<_>>();

    let mut largest_x = 0;
    let mut largest_y = 0;

    s.iter().for_each(|(p1, p2)| {
        if p1.x > largest_x {
            largest_x = p1.x
        }
        if p1.y > largest_y {
            largest_y = p1.y
        }
        if p2.x > largest_x {
            largest_x = p2.x
        }
        if p2.y > largest_y {
            largest_y = p2.y
        }
    });

    let mut matrix = vec![vec![0; largest_x as usize + 1]; largest_y as usize + 1];

    s.iter().for_each(|(p1, p2)| {
        for y in p1.y..p2.y + 1 {
            for x in p1.x..p2.x + 1 {
                matrix[y as usize][x as usize] += 1;
            }
        }
    });

    matrix.iter().flatten().filter(|v| **v > 1).count()
}

pub fn part2() -> usize {
    let s = load_to_rows_and_pattern("input/day5.txt", |c| c == ' ');
    let s = s
        .map(|r| {
            let mut r = r.filter(|s| s != "->");
            let p1 = Pos::from_str(&r.next().unwrap());
            let p2 = Pos::from_str(&r.next().unwrap());
            (p1, p2)
        })
        .collect::<Vec<_>>();

    let mut largest_x = 0;
    let mut largest_y = 0;

    s.iter().for_each(|(p1, p2)| {
        if p1.x > largest_x {
            largest_x = p1.x
        }
        if p1.y > largest_y {
            largest_y = p1.y
        }
        if p2.x > largest_x {
            largest_x = p2.x
        }
        if p2.y > largest_y {
            largest_y = p2.y
        }
    });
    let mut matrix = vec![vec![0; largest_x as usize + 1]; largest_y as usize + 1];

    s.iter()
        .filter(|(p1, p2)| p1.is_45(p2) || p1.is_hoz_ver(p2))
        .for_each(|ps| {
            let (p1, p2) = ps;

            let x_mod = if p1.x == p2.x {
                0
            } else if p1.x < p2.x {
                1
            } else {
                -1
            };
            let y_mod = if p1.y == p2.y {
                0
            } else if p1.y < p2.y {
                1
            } else {
                -1
            };

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
