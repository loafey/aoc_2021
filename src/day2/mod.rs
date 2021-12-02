use aoc_2021::load_to_pattern;

pub fn part1() -> i32 {
    let mut hoz = 0;
    let mut dep = 0;
    let mut direction = String::new();
    load_to_pattern("input/day2.txt", |c| c == ' ').for_each(|w| match w.parse::<i32>() {
        Ok(v) => match &direction[..] {
            "forward" => hoz += v,
            "down" => dep += v,
            "up" => dep -= v,
            _ => {}
        },
        Err(_) => direction = w,
    });

    hoz * dep
}

pub fn part2() -> i32 {
    let mut hoz = 0;
    let mut dep = 0;
    let mut aim = 0;
    let mut direction = String::new();
    load_to_pattern("input/day2.txt", |c| c == ' ').for_each(|w| match w.parse::<i32>() {
        Ok(v) => match &direction[..] {
            "forward" => {
                hoz += v;
                dep += v * aim;
            }
            "down" => aim += v,
            "up" => aim -= v,
            _ => {}
        },
        Err(_) => direction = w,
    });

    hoz * dep
}
