use crate::load;

#[derive(Debug)]
struct Board {
    pub vals: Vec<Vec<i32>>,
    pub found: [[bool; 5]; 5],
}

pub fn part1() -> i32 {
    let s = load("input/day4.txt");
    let mut all_input = s.split("\n\n").collect::<Vec<_>>();
    let numbers = all_input
        .remove(0)
        .split(',')
        .map(|n| n.parse::<i32>().unwrap());

    let mut boards = vec![];

    for b in all_input {
        let vals = b
            .split('\n')
            .map(|r| {
                r.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|w| w.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        let found = [[false; 5]; 5];

        boards.push(Board { vals, found })
    }

    for n in numbers {
        for b in boards.iter_mut() {
            for y in 0..b.vals.len() {
                for x in 0..b.vals[y].len() {
                    if b.vals[y][x] == n {
                        b.found[y][x] = true;
                    }
                }
            }
        }

        for b in &mut boards {
            for r in b.found {
                let mut winner = true;
                for x in r {
                    winner &= x;
                }
                if winner {
                    let mut values_to_remove = vec![];
                    for y in 0..b.vals.len() {
                        for x in 0..b.vals[y].len() {
                            if b.found[y][x] {
                                values_to_remove.push(b.vals[y][x]);
                            }
                        }
                    }
                    b.vals.iter_mut().for_each(|r| {
                        let mut n = r
                            .iter()
                            .filter(|v| !values_to_remove.contains(v))
                            .copied()
                            .collect();
                        r.clear();
                        r.append(&mut n);
                    });
                    return b.vals.iter().map(|v| v.iter().sum::<i32>()).sum::<i32>() * n;
                }
            }

            for c in 0..b.found[0].len() {
                let mut winner = true;
                for y in 0..b.found.len() {
                    winner &= b.found[y][c];
                }
                if winner {
                    let mut values_to_remove = vec![];
                    for y in 0..b.vals.len() {
                        for x in 0..b.vals[y].len() {
                            if b.found[y][x] {
                                values_to_remove.push(b.vals[y][x]);
                            }
                        }
                    }
                    b.vals.iter_mut().for_each(|r| {
                        let mut n = r
                            .iter()
                            .filter(|v| !values_to_remove.contains(v))
                            .copied()
                            .collect();
                        r.clear();
                        r.append(&mut n);
                    });
                    return b.vals.iter().map(|v| v.iter().sum::<i32>()).sum::<i32>() * n;
                }
            }
        }
    }

    0
}

pub fn part2() -> i32 {
    let s = load("input/day4.txt");
    let mut all_input = s.split("\n\n").collect::<Vec<_>>();
    let numbers = all_input
        .remove(0)
        .split(',')
        .map(|n| n.parse::<i32>().unwrap());
    let num_vec = numbers.clone().collect::<Vec<_>>();

    let mut boards = vec![];

    for b in all_input {
        let vals = b
            .split('\n')
            .map(|r| {
                r.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|w| w.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        let found = [[false; 5]; 5];

        boards.push(Board { vals, found })
    }

    for (p, n) in numbers.enumerate() {
        for b in boards.iter_mut() {
            for y in 0..b.vals.len() {
                for x in 0..b.vals[y].len() {
                    if b.vals[y][x] == n {
                        b.found[y][x] = true;
                    }
                }
            }
        }

        let mut winner = false;
        let mut remove = vec![];
        let mut i = 0;
        for b in boards.iter() {
            for r in b.found {
                let mut w = true;
                for x in r {
                    w &= x;
                }
                winner = w;
                if winner {
                    break;
                }
            }

            if !winner {
                for c in 0..b.found[0].len() {
                    let mut w = true;
                    for y in 0..b.found.len() {
                        w &= b.found[y][c];
                    }
                    winner = w;
                    if winner {
                        break;
                    }
                }
            }

            if winner {
                remove.push(i);
            }

            i += 1;
        }

        let mut modd = 0;
        for r in remove {
            if boards.len() == 1 || (num_vec.len() - 1 == p) {
                let b = &mut boards[r];

                println!("{:?}", b.found);
                let mut values_to_remove = vec![];
                for y in 0..b.vals.len() {
                    for x in 0..b.vals[y].len() {
                        if b.found[y][x] {
                            values_to_remove.push(b.vals[y][x]);
                        }
                    }
                }
                b.vals.iter_mut().for_each(|r| {
                    let mut n = r
                        .iter()
                        .filter(|v| !values_to_remove.contains(v))
                        .copied()
                        .collect();
                    r.clear();
                    r.append(&mut n);
                });
                return b.vals.iter().map(|v| v.iter().sum::<i32>()).sum::<i32>() * n;
            } else {
                boards.remove(r - modd);
                modd += 1;
            }
        }
    }

    0
}
