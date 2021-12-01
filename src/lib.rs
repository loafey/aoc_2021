#![allow(unused)]
use std::{
    fmt::{Debug, Display},
    fs,
    path::Path,
    time::Instant,
    vec::IntoIter,
};

pub fn print_line() {
    println!("+--------+------------------+------------------+----------------------------------+");
}
pub fn pretty_print<A: Display, B: Display>(day: usize, p1: fn() -> A, p2: fn() -> B) {
    let timer = Instant::now();
    let v1 = p1();
    let t1 = timer.elapsed().as_secs_f32();

    let timer = Instant::now();
    let v2 = p2();
    let t2 = timer.elapsed().as_secs_f32();

    println!(
        "| {0: <6} | {1: <16} | {2: <16} | {3: <32} |",
        day,
        v1,
        v2,
        format!("{:.4} = {:.4} + {:.4}", t1 + t2, t1, t2)
    );
}

fn load<P: AsRef<Path> + Debug + Copy>(path: P) -> String {
    #[allow(clippy::expect_fun_call)]
    fs::read_to_string(path).expect(&format!("Failed to find file {:?}!", path))
}

pub fn load_to_matrix<A, B, C>(path: A) -> IntoIter<IntoIter<char>>
where
    A: AsRef<Path> + Debug + Copy,
    B: FromIterator<char>,
    C: FromIterator<B>,
{
    load(path)
        .split('\n')
        .map(|r| r.chars().collect::<Vec<_>>().into_iter())
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn load_to_rows<A>(path: A) -> IntoIter<String>
where
    A: AsRef<Path> + Debug + Copy,
{
    load(path)
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn load_to_rows_and_pattern<A, P>(path: A, p: P) -> IntoIter<IntoIter<String>>
where
    A: AsRef<Path> + Debug + Copy,
    P: Fn(char) -> bool + Copy,
{
    load(path)
        .split('\n')
        .map(|s| {
            s.split(p)
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn load_to_pattern<A, P>(path: A, p: P) -> IntoIter<String>
where
    A: AsRef<Path> + Debug + Copy,
    P: Fn(char) -> bool + Copy,
{
    load(path)
        .split(|c| c == '\n' || p(c))
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .into_iter()
}
