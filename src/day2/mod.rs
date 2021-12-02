use aoc_2021::load_to_rows_and_pattern;

pub fn part1() -> i32 {
    let mut hoz = 0;
    let mut dep = 0;
    let mut direction: Option<String> = None;
    load_to_rows_and_pattern("input/day2.txt", |c| c == ' ').for_each(|r| {
        r.for_each(|s| {
            if let Ok(p) = s.parse::<i32>() {
                match &direction.as_ref().unwrap()[..] {
                    "forward" => {
                        hoz += p;
                    }
                    "down" => dep += p,
                    "up" => dep -= p,
                    _ => {}
                }
            } else {
                direction = Some(s);
            }
        })
    });

    hoz * dep
}

pub fn part2() -> i32 {
    let mut hoz = 0;
    let mut dep = 0;
    let mut aim = 0;
    let mut direction: Option<String> = None;
    load_to_rows_and_pattern("input/day2.txt", |c| c == ' ').for_each(|r| {
        r.for_each(|s| {
            if let Ok(p) = s.parse::<i32>() {
                match &direction.as_ref().unwrap()[..] {
                    "forward" => {
                        hoz += p;
                        dep += p * aim;
                    }
                    "down" => aim += p,
                    "up" => aim -= p,
                    _ => {}
                }
            } else {
                direction = Some(s);
            }
        })
    });

    hoz * dep
}
