use lib::benchmark_function as bf;
pub use lib::*;

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
    let benchmarks = [
        bf(1, day1::part1, day1::part2),
        bf(2, day2::part1, day2::part2),
        bf(3, day3::part1, day3::part2),
        bf(4, day4::part1, day4::part2),
        bf(5, day5::part1, day5::part2),
        bf(6, day6::part1, day6::part2),
        bf(7, day7::part1, day7::part2),
        bf(8, day8::part1, day8::part2),
        bf(9, day9::part1, day9::part2),
        bf(10, day10::part1, day10::part2),
        bf(11, day11::part1, day11::part2),
        bf(12, day12::part1, day12::part2),
        bf(13, day13::part1, day13::part2),
        bf(14, day14::part1, day14::part2),
        bf(15, day15::part1, day15::part2),
        bf(16, day16::part1, day16::part2),
        bf(17, day17::part1, day17::part2),
        bf(18, day18::part1, day18::part2),
        bf(19, day19::part1, day19::part2),
        bf(20, day20::part1, day20::part2),
        bf(21, day21::part1, day21::part2),
        bf(22, day22::part1, day22::part2),
        bf(23, day23::part1, day23::part2),
        bf(24, day24::part1, day24::part2),
        bf(25, day25::part1, day25::part2),
    ];

    pretty_print_benchmarks(
        "Welcome to my 2021 Advent of Code solution thing! 🤙🤠🤙",
        &benchmarks,
    );
}
