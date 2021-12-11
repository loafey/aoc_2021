use std::vec::IntoIter;

use aoc_lib::load_to_matrix;

fn get_neighbours(x: usize, y: usize, max_x: usize, max_y: usize) -> IntoIter<(usize, usize)> {
    let x = x as i32;
    let y = y as i32;
    let max_x = max_x as i32;
    let max_y = max_y as i32;

    [
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
    ]
    .into_iter()
    .filter(|(x, y)| !(*x < 0 || *y < 0 || *x >= max_x || *y >= max_y))
    .map(|(x, y)| (x as usize, y as usize))
    .collect::<Vec<_>>()
    .into_iter()
}

fn squider(matrix: &mut [Vec<i32>]) -> i32 {
    let mut flashes = 0;
    matrix.iter_mut().for_each(|r| {
        #[allow(clippy::needless_range_loop)]
        for i in 0..r.len() {
            r[i] += 1;
        }
    });

    let mut buffer = vec![];
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] > 9 {
                matrix[y][x] = 0;
                flashes += 1;

                buffer.append(
                    &mut get_neighbours(x, y, matrix[0].len(), matrix.len())
                        .filter(|(x, y)| matrix[*y][*x] != 0)
                        .collect(),
                );
            }
        }
    }
    while !buffer.is_empty() {
        let (x, y) = buffer[0];

        if matrix[y][x] != 0 {
            matrix[y][x] += 1;
        }
        if matrix[y][x] > 9 {
            matrix[y][x] = 0;
            flashes += 1;
            buffer.append(
                &mut get_neighbours(x, y, matrix[0].len(), matrix.len())
                    .filter(|(x, y)| matrix[*y][*x] != 0)
                    .collect(),
            );
        }

        buffer.remove(0);
    }
    flashes
}

fn same(matrix: &[Vec<i32>]) -> bool {
    let fst = matrix[0][0];
    let mut same = true;
    matrix.iter().flatten().for_each(|v| same &= fst == *v);
    same
}

pub fn part1() -> i32 {
    let mut m = load_to_matrix("input/day11example.txt")
        .map(|r| {
            r.map(|v| v.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += squider(&mut m);
    }

    flashes
}

pub fn part2() -> usize {
    let mut m = load_to_matrix("input/day11.txt")
        .map(|r| {
            r.map(|v| v.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut pos = 0;
    for i in 0..usize::MAX {
        squider(&mut m);
        if same(&m) {
            pos = i + 1;
            break;
        }
    }
    pos
}
