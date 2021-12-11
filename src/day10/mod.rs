use std::vec::IntoIter;

use aoc_lib::load_to_matrix;
use interior_mutability_pointer::Imp;

#[derive(Clone)]
struct Chunk {
    char: char,
    children: Vec<Imp<Chunk>>,
    parent: Option<Imp<Chunk>>,
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string_helper(true))
    }
}

impl Chunk {
    pub fn new(char: char) -> Self {
        Self {
            char,
            children: Vec::new(),
            parent: None,
        }
    }

    pub fn assigned(char: char, assigned: Imp<Chunk>) -> Self {
        Self {
            char,
            children: Vec::new(),
            parent: Some(assigned),
        }
    }

    pub fn match_end(&self, char: char) -> bool {
        match self.char {
            '(' => ')' == char,
            '{' => '}' == char,
            '[' => ']' == char,
            '<' => '>' == char,
            _ => false,
        }
    }

    pub fn corrupt(&self) -> bool {
        let mut r = !matches!(self.char, '(' | '{' | '[' | '<');

        self.children.iter().for_each(|c| r |= c.corrupt());
        r
    }

    pub fn first_corrupt(&self) -> Option<char> {
        let alright = matches!(self.char, '(' | '{' | '[' | '<');

        if alright {
            let mut ans = None;
            for c in &self.children {
                let a = c.first_corrupt();
                if a.is_some() {
                    ans = a;
                    break;
                }
            }
            ans
        } else {
            Some(self.char)
        }
    }

    pub fn parse_line(r: &mut IntoIter<char>) -> Imp<Self> {
        let fst = r.next().unwrap();
        let mut parent_chunk = Imp::new(Chunk::new(fst));
        let mut current = Imp::new(Chunk::assigned(fst, Imp::clone(&parent_chunk)));
        parent_chunk.children.push(current.clone());
        r.for_each(|c| {
            if current.match_end(c) {
                if let Some(p) = &current.parent {
                    current = Imp::clone(p);
                }
            } else {
                current = Imp::new(Chunk::assigned(c, Imp::clone(&current)));
                if let Some(mut p) = current.parent.clone() {
                    p.children.push(current.clone())
                }
            }
        });
        parent_chunk
    }

    fn to_string_helper(&self, first: bool) -> String {
        let end = match self.char {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            '<' => '>',
            _ => '-',
        };
        let middle = {
            let mut s = String::new();
            self.children
                .iter()
                .for_each(|child| s = format!("{}{}", s, child.to_string_helper(false)));
            s
        };
        if first {
            middle
        } else {
            format!("{}{}{}", self.char, middle, end)
        }
    }
}

pub fn part1() -> i32 {
    let mut paran = 0;
    let mut square = 0;
    let mut curly = 0;
    let mut arrow = 0;

    load_to_matrix("input/day10.txt").for_each(|mut r| {
        let parent_chunk = Chunk::parse_line(&mut r);

        if parent_chunk.corrupt() {
            if let Some(c) = parent_chunk.first_corrupt() {
                match c {
                    ')' => paran += 1,
                    ']' => square += 1,
                    '}' => curly += 1,
                    '>' => arrow += 1,
                    _ => {}
                }
            }
        }
    });

    (paran * 3) + (square * 57) + (curly * 1197) + (arrow * 25137)
}

pub fn part2() -> i128 {
    let mut v = load_to_matrix("input/day10.txt")
        .filter_map(|mut r| {
            let raw_str = r.clone().collect::<String>();

            let parent_chunk = Chunk::parse_line(&mut r);
            if !parent_chunk.corrupt() {
                let mut s = parent_chunk.to_string();

                for c in raw_str.chars() {
                    if c == s.chars().next().unwrap() {
                        s.remove(0);
                    }
                }

                let mut score = 0;
                s.chars().into_iter().for_each(|c| match c {
                    ')' => score = (score * 5) + 1,
                    ']' => score = (score * 5) + 2,
                    '}' => score = (score * 5) + 3,
                    '>' => score = (score * 5) + 4,
                    _ => {}
                });
                Some(score)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    v.sort_unstable();
    v[v.len() / 2]
}
