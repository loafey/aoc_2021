use crate::math::Matrix;
use std::{fmt::Debug, fs, path::Path, vec::IntoIter};

fn load<P: AsRef<Path> + Debug + Copy>(path: P) -> String {
    #[allow(clippy::expect_fun_call)]
    fs::read_to_string(path).expect(&format!("Failed to find file {:?}!", path))
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

pub fn load_to_rows_and_pattern<A, P>(path: A, p: P) -> Matrix<String>
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

pub fn load_to_matrix<A>(path: A) -> Matrix<char>
where
    A: AsRef<Path> + Debug + Copy,
{
    load(path)
        .split('\n')
        .map(|r| r.chars().collect::<Vec<_>>().into_iter())
        .collect::<Vec<_>>()
        .into_iter()
}
