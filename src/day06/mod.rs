use aoc_lib::load_to_pattern;

pub fn part1() -> usize {
    solver(80)
}

pub fn part2() -> usize {
    solver(256)
}

fn solver(end_day: usize) -> usize {
    let mut fishies = [0i128; 10];
    load_to_pattern("input/day6.txt", |c| c == ',')
        .for_each(|v| fishies[v.parse::<usize>().unwrap()] += 1);
    for _ in 0..end_day {
        for i in 0..fishies.len() {
            if i == 0 {
                fishies[9] += fishies[i];
                fishies[7] += fishies[i];
                fishies[0] = 0;
            } else {
                fishies[i - 1] += fishies[i];
                fishies[i] = 0;
            }
        }
    }
    fishies.iter().map(|f| *f as usize).sum()
}
