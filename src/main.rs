use aoc_2021::{print_func, print_line};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod lib;
fn main() {
    println!("┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│           Welcome to my 2021 Advent of Code solution thing! 🤙🤠🤙           │");
    println!("├─────┬──────────────────┬──────────────────┬──────────────────────────────────┤");
    println!(
        "│ Day │ {0: <16} │ {1: <16} │ {2: <32} │",
        "Part 1", "Part 2", "Time(s) = P1 + P2"
    );
    print_line();

    let tt = vec![
        print_func(1, day1::part1, day1::part2),
        print_func(2, day2::part1, day2::part2),
        print_func(3, day3::part1, day3::part2),
        print_func(4, day4::part1, day4::part2),
        print_func(5, day5::part1, day5::part2),
        print_func(6, day6::part1, day6::part2),
        print_func(7, day7::part1, day7::part2),
        print_func(8, day8::part1, day8::part2),
        print_func(9, day9::part1, day9::part2),
        print_func(10, day10::part1, day10::part2),
        print_func(11, day11::part1, day11::part2),
        print_func(12, day12::part1, day12::part2),
        print_func(13, day13::part1, day13::part2),
        print_func(14, day14::part1, day14::part2),
        print_func(15, day15::part1, day15::part2),
        print_func(16, day16::part1, day16::part2),
        print_func(17, day17::part1, day17::part2),
        print_func(18, day18::part1, day18::part2),
        print_func(19, day19::part1, day19::part2),
        print_func(20, day20::part1, day20::part2),
        print_func(21, day21::part1, day21::part2),
        print_func(22, day22::part1, day22::part2),
        print_func(23, day23::part1, day23::part2),
        print_func(24, day24::part1, day24::part2),
        print_func(25, day25::part1, day25::part2),
    ]
    .iter()
    .sum::<f64>();

    println!("├─────┴──────────────────┴──────────────────┴──────────────────────────────────┤");
    println!("│ Total time: {0: <64} │", format!("{:.5}m", tt / 60.0));
    println!("└──────────────────────────────────────────────────────────────────────────────┘");
}
