use aoc_lib::load_to_pattern;

pub fn part1() -> i32 {
    let mut v = load_to_pattern("input/day7.txt", |c| c == ',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    v.sort_unstable();

    let median = v[v.len() / 2];

    let mut consume = 0;
    v.iter().for_each(|i| consume += (*i - median).abs());
    consume
}

pub fn part2() -> i32 {
    let v = load_to_pattern("input/day7.txt", |c| c == ',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let avg = v.iter().sum::<i32>() / v.len() as i32;
    let mut consume = 0;
    v.iter().for_each(|v| {
        let p = (*v - avg).abs();
        for p in 0..p + 1 {
            consume += p;
        }
    });
    consume
}
