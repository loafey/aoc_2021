use std::vec::IntoIter;

pub type Matrix<A> = IntoIter<IntoIter<A>>;
pub fn rotate_90deg<A: Copy>(m: Matrix<A>) -> Matrix<A> {
    let m = m
        .map(|r| {
            let mut r = r.collect::<Vec<_>>();
            r.reverse();
            r
        })
        .collect::<Vec<_>>();
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

pub fn transpose<A: Copy>(m: Matrix<A>) -> Matrix<A> {
    let m = m.map(|r| r.collect::<Vec<_>>()).collect::<Vec<_>>();
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
