use aoc_2021::load_to_pattern;

// 0.0024
pub fn part1() -> i32 {
    let mut hoz = 0;
    let mut dep = 0;
    let mut direction = String::new();
    load_to_pattern("input/day2.txt", |c| c == ' ').for_each(|w| match w.parse::<i32>() {
        Ok(v) => {
            hoz += (&direction[..] == "forward") as i32 * v;
            dep += (&direction[..] == "down") as i32 * v;
            dep -= (&direction[..] == "up") as i32 * v;
        }
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
        Ok(v) => {
            hoz += (&direction[..] == "forward") as i32 * v;
            dep += (&direction[..] == "forward") as i32 * v * aim;
            aim += (&direction[..] == "down") as i32 * v;
            aim -= (&direction[..] == "up") as i32 * v;
        }
        Err(_) => direction = w,
    });

    hoz * dep
}
