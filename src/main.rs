use aoc_2021::{pretty_print, print_line};

mod day1;
mod lib;
fn main() {
    print_line();
    println!(
        "| {0: <6} | {1: <16} | {2: <16} | {3: <32} |",
        "Day", "Part 1", "Part 2", "Time(s) = P1 + P2"
    );
    print_line();

    pretty_print(1, day1::part1, day1::part2);

    print_line();
}
