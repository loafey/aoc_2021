use aoc_lib::load_to_pattern;

pub fn part1() -> i32 {
    let v = load_to_pattern("input/day7.txt", |c| c == ',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min = *v
        .iter()
        .reduce(|accum, item| if accum < item { accum } else { item })
        .unwrap() as usize;
    let max = *v
        .iter()
        .reduce(|accum, item| if accum >= item { accum } else { item })
        .unwrap() as usize;

    let mut crab_power = vec![];
    for i in min..max {
        let mut consume = 0;
        v.iter().for_each(|v| consume += (*v - i as i32).abs());
        crab_power.push(consume);
    }
    //crab_power.iter().for_each(|c| println!("{:?}", c));

    crab_power
        .into_iter()
        .reduce(|accum, item| if accum < item { accum } else { item })
        .unwrap_or_default()
}

pub fn part2() -> i32 {
    let v = load_to_pattern("input/day7.txt", |c| c == ',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min = *v
        .iter()
        .reduce(|accum, item| if accum < item { accum } else { item })
        .unwrap() as usize;
    let max = *v
        .iter()
        .reduce(|accum, item| if accum >= item { accum } else { item })
        .unwrap() as usize;

    let mut crab_power = vec![];
    for i in min..max {
        let mut consume = 0;
        v.iter().for_each(|v| {
            let p = (*v - i as i32).abs();
            for p in 0..p + 1 {
                consume += p;
            }
        });
        crab_power.push(consume);
    }
    //crab_power.iter().for_each(|c| println!("{:?}", c));

    crab_power
        .into_iter()
        .reduce(|accum, item| if accum < item { accum } else { item })
        .unwrap_or_default()
}
