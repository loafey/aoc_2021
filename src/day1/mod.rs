use aoc_2021::load_to_pattern;

pub fn part1() -> i32 {
    let mut current = 0;
    let mut incs = 0;
    load_to_pattern("input/day1.txt", |c| c == ' ')
        .map(|c| c.parse::<i32>().unwrap())
        .enumerate()
        .for_each(|(i, v)| {
            current = v * (i == 0) as i32 + current * (i != 0) as i32;
            incs += (v > current) as i32;
            current = v;
        });

    incs
}

pub fn part2() -> i32 {
    let nums: Vec<_> = load_to_pattern("input/day1.txt", |c| c == ' ')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let sum = |i: usize| nums[i] + nums[i + 1] + nums[i + 2];

    let mut incs = 0;
    let mut old_sum = sum(0);

    for i in 0..nums.len() - 2 {
        incs += (old_sum < sum(i)) as i32;
        old_sum = sum(i);
    }

    incs
}