use std::collections::HashMap;

use aoc_lib::load_to_rows_and_pattern;
use interior_mutability_pointer::Imp;

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

pub fn part1() -> String {
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
            a_ref.roads.push(b);
            b_ref.roads.push(a);
        });

    let mut buffer = vec!["start".to_string()];
    let mut path = vec![];

    while !buffer.is_empty() {
        let c = buffer[0].clone();

        if c != "end" {
            caves.get(&c).unwrap().roads.iter().for_each(|c| {
                buffer.push(c.clone());
            });
        } else {
            println!("{:?}", buffer);
            break;
        }

        buffer.remove(0);
    }

    String::new()
}

pub fn part2() -> String {
    String::new()
}
