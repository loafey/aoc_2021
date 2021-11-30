#![allow(unused)]
use std::{fmt::Debug, fs, path::Path};

fn load<P: AsRef<Path> + Debug + Copy>(path: P) -> String {
    #[allow(clippy::expect_fun_call)]
    fs::read_to_string(path).expect(&format!("Failed to find file {:?}!", path))
}

pub fn load_to_matrix<A, B, C>(path: A) -> C
where
    A: AsRef<Path> + Debug + Copy,
    B: FromIterator<char>,
    C: FromIterator<B>,
{
    load(path)
        .split('\n')
        .map(|r| r.chars().collect::<B>())
        .collect::<C>()
}

pub fn load_to_rows<A, B>(path: A) -> B
where
    A: AsRef<Path> + Debug + Copy,
    B: FromIterator<String>,
{
    load(path).split('\n').map(|s| s.to_string()).collect()
}

pub fn load_to_rows_and_pattern<A, B, C, P>(path: A, p: P) -> C
where
    A: AsRef<Path> + Debug + Copy,
    B: FromIterator<String>,
    C: FromIterator<B>,
    P: Fn(char) -> bool + Copy,
{
    load(path)
        .split('\n')
        .map(|s| s.split(p).map(|s| s.to_string()).collect::<B>())
        .collect::<C>()
}

pub fn load_to_pattern<A, C, P>(path: A, p: P) -> C
where
    A: AsRef<Path> + Debug + Copy,
    C: FromIterator<String>,
    P: Fn(char) -> bool + Copy,
{
    load(path)
        .split('\n')
        .map(|r| {
            r.to_string()
                .split(p)
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}
