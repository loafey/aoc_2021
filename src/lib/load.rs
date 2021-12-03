use std::{fmt::Debug, fs, path::Path, vec::IntoIter};

fn load<P: AsRef<Path> + Debug + Copy>(path: P) -> String {
    #[allow(clippy::expect_fun_call)]
    fs::read_to_string(path).expect(&format!("Failed to find file {:?}!", path))
}

pub fn load_to_matrix<A>(path: A) -> IntoIter<IntoIter<char>>
where
    A: AsRef<Path> + Debug + Copy,
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

pub fn load_to_matrix_transpose<A>(path: A) -> IntoIter<IntoIter<char>>
where
    A: AsRef<Path> + Debug + Copy,
{
    let m: Vec<Vec<_>> = load_to_matrix(path).map(|v| v.collect()).collect();
    let mut collums = vec![];
    for y in 0..m[0].len() {
        let mut collumn = vec![];
        for x in &m {
            collumn.push(x[y]);
        }
        collums.push(collumn.into_iter());
    }
    collums.into_iter()
}

pub fn load_to_matrix_90deg<A>(path: A) -> IntoIter<IntoIter<char>>
where
    A: AsRef<Path> + Debug + Copy,
{
    let m: Vec<Vec<_>> = load_to_matrix(path)
        .map(|v| {
            let mut v = v.collect::<Vec<_>>();
            v.reverse();
            v
        })
        .collect(); // Reversed;
    let mut collums = vec![];
    for y in 0..m[0].len() {
        let mut collumn = vec![];
        for x in &m {
            collumn.push(x[y]);
        }
        collums.push(collumn.into_iter());
    }
    collums.into_iter()
}
