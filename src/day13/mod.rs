use aoc_lib::{load, math::transpose_vec};

#[derive(Debug)]
enum Dir {
    X(usize),
    Y(usize),
    Gof,
}
impl Dir {
    pub fn new(d: &str, amount: usize) -> Self {
        match d {
            "x" | "X" => Dir::X(amount),
            "y" | "Y" => Dir::Y(amount),
            _ => Dir::Gof,
        }
    }

    pub fn parse_directions(str: &str) -> Vec<Dir> {
        str.split('\n')
            .into_iter()
            .map(|r| {
                let mut v = r.split(' ').nth(2).unwrap().split('=');

                let c = v.next().unwrap();
                let v = v.next().unwrap().parse().unwrap();

                Dir::new(c, v)
            })
            .collect()
    }
}

fn parse_map(data: &str) -> Vec<Vec<char>> {
    let mut largest_x = 0;
    let mut largest_y = 0;
    let coords = data
        .split('\n')
        .map(|r| {
            let mut s = r.split(',');
            let x = s.next().unwrap().parse::<usize>().unwrap();
            let y = s.next().unwrap().parse::<usize>().unwrap();

            if x >= largest_x {
                largest_x = x + 1;
            }
            if y >= largest_y {
                largest_y = y + 1;
            }
            (x, y)
        })
        .collect::<Vec<_>>();

    let mut board = vec![vec!['.'; largest_x]; largest_y];
    for (x, y) in coords {
        board[y][x] = '#';
    }

    board
}

fn flip(data: &mut Vec<Vec<char>>, v: usize) {
    let mut p1 = data[0..v].to_vec();
    let mut p2 = data[v + data.len() % 2..data.len()].to_vec();

    p2.reverse();

    p2.iter().enumerate().for_each(|(y, r)| {
        r.iter().enumerate().for_each(|(x, _)| {
            if p2[y][x] == '#' {
                p1[y][x] = p2[y][x];
            }
        })
    });

    let flippert = data.len() % 2 == 0;
    if flippert {
        let l = p1.len() - 1;
        for x in 0..p1[l].len() {
            p1[l][x] = '.';
        }
    }

    data.clear();
    data.append(&mut p1);
}

fn fold(data: &mut Vec<Vec<char>>, dir: &Dir) {
    match dir {
        Dir::X(v) => {
            transpose_vec(data);
            flip(data, *v);
            transpose_vec(data);
        }
        Dir::Y(v) => flip(data, *v),
        Dir::Gof => panic!("What are we doing here?"),
    }
}

pub fn part1() -> usize {
    let i = load("input/day13example.txt");
    let mut splot = i.split("\n\n");

    let mut data = parse_map(splot.next().unwrap());
    let instructions = Dir::parse_directions(splot.next().unwrap());

    data.iter().for_each(|r| {
        let mut s = String::new();
        r.iter().for_each(|c| s.push(*c));
        println!("{}", s);
    });
    /*instructions.into_iter().for_each(|i| {
        fold(&mut data, &i);

        println!("- {:<3?} -", i);
        data.iter().for_each(|r| {
            let mut s = String::new();
            r.iter().for_each(|c| s.push(*c));
            println!("{}", s);
        });
    });**/
    println!("--");
    fold(&mut data, &instructions[0]);
    data.iter().for_each(|r| {
        let mut s = String::new();
        r.iter().for_each(|c| s.push(*c));
        println!("{}", s);
    });

    counter(&mut data)
}

pub fn counter(data: &mut Vec<Vec<char>>) -> usize {
    data.iter().flatten().filter(|c| **c == '#').count()
}

pub fn part2() -> String {
    String::new()
}
