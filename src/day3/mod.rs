use crate::load_to_matrix;

pub fn part1() -> i32 {
    let m: Vec<Vec<_>> = load_to_matrix("input/day3.txt")
        .map(|v| v.collect())
        .collect();

    let mut gamma = String::new();
    let mut epsilion = String::new();
    for y in 0..m[0].len() {
        let mut zero_amount = 0;
        let mut one_amount = 0;
        for x in 0..m.len() {
            let v = m[x][y];
            match v {
                '0' => zero_amount += 1,
                '1' => one_amount += 1,
                _ => {}
            }
        }
        let most_common = if zero_amount > one_amount { '0' } else { '1' };
        let least_common = if zero_amount > one_amount { '1' } else { '0' };
        gamma.push(most_common);
        epsilion.push(least_common);
    }
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilion = i32::from_str_radix(&epsilion, 2).unwrap();

    gamma * epsilion
}

pub fn part2() -> i32 {
    let m: Vec<Vec<_>> = load_to_matrix("input/day3.txt")
        .map(|v| v.collect())
        .collect();

    let mut oxygen_list = vec![];
    let mut c02_list = vec![];

    #[allow(clippy::never_loop)]
    for y in 0..m[0].len() {
        let mut zero_amount = 0;
        let mut one_amount = 0;
        for x in 0..m.len() {
            let v = m[x][y];
            match v {
                '0' => zero_amount += 1,
                '1' => one_amount += 1,
                _ => {}
            }
        }
        let most_common = if zero_amount > one_amount { '0' } else { '1' };
        let least_common = if zero_amount > one_amount { '1' } else { '0' };

        for r in &m {
            if r[0] == most_common {
                oxygen_list.push(r.iter().collect::<Vec<_>>())
            }
            if r[0] == least_common {
                c02_list.push(r.iter().collect::<Vec<_>>())
            }
        }

        break;
    }

    let mut index = 0;
    let len = oxygen_list[0].len();
    while index < len {
        let mut zero_amount = 0;
        let mut one_amount = 0;
        for o in &oxygen_list {
            if *o[index] == '0' {
                zero_amount += 1
            } else {
                one_amount += 1
            }
        }
        let mut most_common = if zero_amount > one_amount { '0' } else { '1' };
        if zero_amount == one_amount {
            most_common = '1';
        }
        oxygen_list = oxygen_list
            .into_iter()
            .filter(|o| *o[index] == most_common)
            .collect();

        index += 1;
        if oxygen_list.len() == 1 {
            break;
        }
    }

    let oxy_winner = oxygen_list[0].clone().into_iter().collect::<String>();

    //
    let mut index = 1;
    let len = c02_list[0].len();
    while index < len {
        let mut zero_amount = 0;
        let mut one_amount = 0;
        for o in &c02_list {
            if *o[index] == '0' {
                zero_amount += 1
            } else {
                one_amount += 1
            }
        }
        let mut least_common = if zero_amount > one_amount { '1' } else { '0' };
        if zero_amount == one_amount {
            least_common = '0';
        }
        c02_list = c02_list
            .into_iter()
            .filter(|o| *o[index] == least_common)
            .collect();

        index += 1;
        if c02_list.len() == 1 {
            break;
        }
    }
    let c02_winner = c02_list[0].clone().into_iter().collect::<String>();

    let oxy = i32::from_str_radix(&oxy_winner, 2).unwrap();
    let c02 = i32::from_str_radix(&c02_winner, 2).unwrap();

    oxy * c02
}
