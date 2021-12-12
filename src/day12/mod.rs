use std::collections::HashMap;

use aoc_lib::load_to_rows_and_pattern;
use interior_mutability_pointer::Imp;

#[derive(Debug)]
enum Path {
    Leaf(String),
    Node(String, Vec<Path>),
}
impl Path {
    pub fn name(&self) -> &String {
        match self {
            Path::Leaf(s) => s,
            Path::Node(s, _) => s,
        }
    }

    pub fn ends(&self) -> usize {
        match self {
            Path::Leaf(s) => (s == "end") as usize,
            Path::Node(_, v) => v.iter().map(|p| p.ends()).sum(),
        }
    }
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.helper(1))
    }
}
impl Path {
    fn helper(&self, tab: usize) -> String {
        let mut t = String::new();
        match &self {
            Path::Leaf(s) => s.clone(),
            Path::Node(s, ps) => {
                ps.iter()
                    .for_each(|p| t = format!("{}\n{}{}", t, "│ ".repeat(tab), p.helper(tab + 1)));
                format!("{}{}", s, t)
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Cave {
    pub big: bool,
    pub roads: Vec<String>,
}
impl Cave {
    pub fn new(big: bool) -> Imp<Self> {
        Imp::new(Self { big, roads: vec![] })
    }
}

pub fn part1() -> usize {
    let s = "input/day12example.txt";
    let mut caves = HashMap::new();
    caves.insert("start".to_string(), Cave::new(true));
    caves.insert("end".to_string(), Cave::new(true));

    load_to_rows_and_pattern(s, |c| c == '-').for_each(|r| {
        r.for_each(|w| match &w[..] {
            "start" => {}
            "end" => {}
            _ => {
                let uppercase = !w.chars().next().unwrap().is_lowercase();
                let cave = Cave::new(uppercase);
                caves.entry(w).or_insert(cave);
            }
        });
    });
    load_to_rows_and_pattern(s, |c| c == '-')
        .map(|mut r| {
            let a = r.next().unwrap();
            let b = r.next().unwrap();
            (a, b)
        })
        .for_each(|(a, b)| {
            let mut a_ref = caves.get(&a).unwrap().clone();
            let mut b_ref = caves.get(&b).unwrap().clone();
            if b != "start" && a != "end" {
                a_ref.roads.push(b.clone());
            }
            if a != "start" && b != "end" {
                b_ref.roads.push(a);
            }
        });

    caves.iter().for_each(|(k, c)| {
        println!("{:<6}|\t{:?}", k, c.roads);
    });

    let start = evolve(Path::Leaf("start".to_string()), vec![], &caves);

    start.ends()
}

pub fn part2() -> String {
    String::new()
}

fn evolve(p: Path, explored: Vec<String>, caves: &HashMap<String, Imp<Cave>>) -> Path {
    let n = p.name();
    let r = &caves.get(n).unwrap().roads;
    if r.is_empty() {
        p
    } else {
        Path::Node(
            n.clone(),
            r.iter()
                .filter_map(|s| {
                    if !explored.contains(s) {
                        let c = caves.get(s).unwrap();
                        let mut n = explored.clone();
                        if !c.big {
                            n.push(s.clone());
                        }
                        Some(evolve(Path::Leaf(s.clone()), n, caves))
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}
