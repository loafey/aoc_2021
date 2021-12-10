#![feature(slice_group_by)]
use aoc_lib::benchmark_function as bf;
pub use aoc_lib::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
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
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let benchmarks = [
        bf(1, day01::part1, day01::part2),
        bf(2, day02::part1, day02::part2),
        bf(3, day03::part1, day03::part2),
        bf(4, day04::part1, day04::part2),
        bf(5, day05::part1, day05::part2),
        bf(6, day06::part1, day06::part2),
        bf(7, day07::part1, day07::part2),
        bf(8, day08::part1, day08::part2),
        bf(9, day09::part1, day09::part2),
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
