use crate::{load_to_matrix, math::transpose};

pub fn part1() -> i32 {
    let mut gamma = String::new();
    let mut epsilion = String::new();
    transpose(load_to_matrix("input/day3.txt")).for_each(|c| {
        let mut zero_amount = 0;
        let mut one_amount = 0;
        c.for_each(|c| {
            zero_amount += (c == '0') as i32;
            one_amount += (c == '1') as i32
        });
        let most_common = if zero_amount > one_amount { '0' } else { '1' };
        let least_common = if zero_amount > one_amount { '1' } else { '0' };
        gamma.push(most_common);
        epsilion.push(least_common);
    });

    i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilion, 2).unwrap()
}

fn scrubby_scrub_scrub(
    start_index: usize,
    zc: char,
    oc: char,
    mut c02_list: Vec<Vec<char>>,
) -> i32 {
    for index in start_index..c02_list[0].len() {
        let mut zero_amount = 0;
        let mut one_amount = 0;
        for o in &c02_list {
            if o[index] == '0' {
                zero_amount += 1
            } else {
                one_amount += 1
            }
        }
        c02_list = c02_list
            .into_iter()
            .filter(|o| o[index] == if zero_amount <= one_amount { zc } else { oc })
            .collect();
        if c02_list.len() == 1 {
            break;
        }
    }

    i32::from_str_radix(&c02_list[0].clone().into_iter().collect::<String>(), 2).unwrap()
}
pub fn part2() -> i32 {
    let m = load_to_matrix("input/day3.txt");
    let mt = transpose(m.clone());

    let most_commons = mt
        .map(|c| {
            let mut zero_amount = 0;
            let mut one_amount = 0;
            c.for_each(|c| {
                zero_amount += (c == '0') as i32;
                one_amount += (c == '1') as i32
            });
            if zero_amount > one_amount {
                '0'
            } else {
                '1'
            }
        })
        .collect::<Vec<_>>();

    let mut oxygen_list = vec![];
    let mut c02_list = vec![];
    m.for_each(|r| {
        let r = r.collect::<Vec<_>>();
        if r[0] == most_commons[0] {
            oxygen_list.push(r);
        } else {
            c02_list.push(r);
        }
    });

    let oxy_winner = scrubby_scrub_scrub(0, '1', '0', oxygen_list);
    let c02_winner = scrubby_scrub_scrub(1, '0', '1', c02_list);
    oxy_winner * c02_winner
}
