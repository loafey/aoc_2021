use aoc_lib::load_to_matrix;

fn get_neigbours(m: &[Vec<i32>], x: usize, y: usize) -> Vec<(i32, usize, usize)> {
    let mut n = vec![];
    if x == 0 {
        n.push((m[y][x + 1], x + 1, y))
    } else if x == m[0].len() - 1 {
        n.push((m[y][x - 1], x - 1, y))
    } else {
        n.push((m[y][x + 1], x + 1, y));
        n.push((m[y][x - 1], x - 1, y))
    }

    if y == 0 {
        n.push((m[y + 1][x], x, y + 1))
    } else if y == m.len() - 1 {
        n.push((m[y - 1][x], x, y - 1))
    } else {
        n.push((m[y - 1][x], x, y - 1));
        n.push((m[y + 1][x], x, y + 1))
    }

    n
}

fn get_middles(m: &[Vec<i32>]) -> Vec<(usize, usize)> {
    m.iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter().enumerate().filter_map(move |(x, v)| {
                let mut n = get_neigbours(m, x, y)
                    .into_iter()
                    .map(|(v, _, _)| v)
                    .collect::<Vec<_>>();
                n.push(m[y][x]);
                let mut c = n.clone();
                c.sort_unstable();
                c.dedup();

                let min = n
                    .into_iter()
                    .reduce(|accum, item| if accum < item { accum } else { item })
                    .unwrap();

                if min == *v && c.len() != 1 {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

pub fn part1() -> i32 {
    let m = load_to_matrix("input/day9.txt")
        .map(|r| {
            r.map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    get_middles(&m)
        .into_iter()
        .map(|(x, y)| m[y][x] + 1)
        .sum::<i32>()
}

pub fn part2() -> usize {
    let m = load_to_matrix("input/day9.txt")
        .map(|r| {
            r.map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut middles = get_middles(&m)
        .iter()
        .map(|(x, y)| (0, *x, *y))
        .collect::<Vec<_>>();

    let mut flow = vec![];
    middles.iter_mut().for_each(|t| {
        let (_, x, y) = t;
        t.0 += 1;
        let mut buffers = vec![];
        let mut done = vec![(*x, *y)];
        flow.push((*x, *y));

        buffers.append(
            &mut get_neigbours(&m, *x, *y)
                .into_iter()
                .filter(|(v, _, _)| *v != 9)
                .collect(),
        );

        while !buffers.is_empty() {
            let (_, x, y) = buffers[0];
            if !done.contains(&(x, y)) {
                t.0 += 1;
                buffers.append(
                    &mut get_neigbours(&m, x, y)
                        .into_iter()
                        .filter(|(v, _, _)| *v != 9)
                        .collect(),
                );
                done.push((x, y));
                flow.push((x, y));
            }
            buffers.remove(0);
        }
    });

    let mut middles = middles.into_iter().map(|(v, _, _)| v).collect::<Vec<_>>();
    middles.sort_unstable();

    let a = middles[middles.len() - 1];
    let b = middles[middles.len() - 2];
    let c = middles[middles.len() - 3];
    println!("{:?}", middles);
    a * b * c
}
