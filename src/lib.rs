#![allow(unused)]
use core::time;
use std::{
    fmt::{Debug, Display},
    fs,
    path::Path,
    time::Instant,
    vec::IntoIter,
};

pub struct FunctionBenchmark {
    day: usize,
    time1: f64,
    time2: f64,
    result1: Box<dyn Display>,
    result2: Box<dyn Display>,
}

pub fn benchmark_function<A: Display + 'static, B: Display + 'static>(
    day: usize,
    p1: fn() -> A,
    p2: fn() -> B,
) -> FunctionBenchmark {
    let timer = Instant::now();
    let v1 = p1();
    let t1 = timer.elapsed().as_secs_f64();

    let timer = Instant::now();
    let v2 = p2();
    let t2 = timer.elapsed().as_secs_f64();
    FunctionBenchmark {
        day,
        time1: t1,
        time2: t2,
        result1: Box::new(v1),
        result2: Box::new(v2),
    }
}

fn calculate_string_lens(v: &[FunctionBenchmark]) -> (usize, usize, usize) {
    let mut part1_len = 0;
    let mut part2_len = 0;
    let mut time_len = 0;
    v.iter().for_each(|f| {
        let l1 = format!("{}", f.result1).len();
        let l2 = format!("{}", f.result2).len();
        if l1 > part1_len {
            part1_len = l1;
        }
        if l2 > part2_len {
            part2_len = l2;
        }
        let t = format!("{:.4} = {:.4} + {:.4}", f.time1 + f.time2, f.time1, f.time2).len();
        if t > time_len {
            time_len = t;
        }
    });
    (part1_len, part2_len, time_len)
}

pub fn pretty_print_benchmarks(nice_text: &str, v: &[FunctionBenchmark]) {
    let (part1_len, part2_len, time_len) = calculate_string_lens(v);

    {
        let total = "─".repeat(part1_len + part2_len + time_len + 14);
        println!("┌{}┐", total);
    }
    {
        let s = nice_text;
        let s_len = s.len();
        let words = s.split_ascii_whitespace();
        let mut fitted_words = vec![];
        let mut current_row = vec![];
        let mut len_counter = 0;
        for s in words {
            len_counter += s.len() + 1;
            if len_counter < part1_len + part2_len + time_len {
                current_row.push(s.to_string());
            } else {
                fitted_words.push(current_row);
                current_row = vec![s.to_string()];
                len_counter = 0;
            }
        }
        fitted_words.push(current_row);
        fitted_words.iter().for_each(|r| {
            let mut row = String::new();

            for (i, u) in r.iter().enumerate() {
                row.push_str(u);
                if i != r.len() - 1 {
                    row.push(' ');
                }
            }

            let mut quantifier = 0;
            for c in row.chars() {
                if c.len_utf16() > 1 {
                    quantifier += c.len_utf16();
                }
            }

            let mut s = (part1_len + part2_len + time_len + 12 + quantifier) - row.len();
            let l = s / 2;
            let mut r = s / 2;
            if s % 2 == 1 {
                r += 1;
            }
            println!("│ {}{}{} │", " ".repeat(l), row, " ".repeat(r));
        });
    }
    {
        let part1 = "═".repeat(part1_len + 2);
        let part2 = "═".repeat(part2_len + 2);
        let time = "═".repeat(time_len + 2);
        println!("╞═════╤{}╤{}╤{}╡", part1, part2, time);
    }
    {
        let part1 = format!("Part 1{}", " ".repeat(part1_len - 6));
        let part2 = format!("Part 2{}", " ".repeat(part2_len - 6));
        let time = format!("Time(s) = P1 + P2{}", " ".repeat(time_len - 17));
        println!("│ Day │ {} │ {} │ {} │", part1, part2, time);
    }
    {
        let part1 = "─".repeat(part1_len + 2);
        let part2 = "─".repeat(part2_len + 2);
        let time = "─".repeat(time_len + 2);
        println!("├─────┼{}┼{}┼{}┤", part1, part2, time);
    }

    let mut tt = 0.0;
    v.iter().for_each(|f| {
        let mut formated_part1 = format!("{}", f.result1);
        let mut formated_part2 = format!("{}", f.result2);
        let mut formated_time =
            format!("{:.4} = {:.4} + {:.4}", f.time1 + f.time2, f.time1, f.time2);

        if formated_part1.len() < part1_len {
            let extra = part1_len - formated_part1.len();
            formated_part1 = format!("{}{}", formated_part1, " ".repeat(extra));
        }

        if formated_part2.len() < part2_len {
            let extra = part2_len - formated_part2.len();
            formated_part2 = format!("{}{}", formated_part2, " ".repeat(extra));
        }

        if formated_time.len() < time_len {
            let extra = time_len - formated_time.len();
            formated_time = format!("{}{}", formated_time, " ".repeat(extra));
        }

        let day = if f.day < 10 {
            format!(" {0: <2}", f.day)
        } else {
            format!("{0: <3}", f.day)
        };

        if !(formated_part1.trim().is_empty() && formated_part2.trim().is_empty()) {
            println!(
                "│ {} │ {} │ {} │ {} │",
                day, formated_part1, formated_part2, formated_time
            );
        }

        tt += f.time1 + f.time2;
    });

    {
        let part1 = "─".repeat(part1_len + 2);
        let part2 = "─".repeat(part2_len + 2);
        let time = "─".repeat(time_len + 2);
        println!("├─────┴{}┴{}┴{}┤", part1, part2, time);
    }
    {
        let s = format!("Total time: {:.8}s", tt);
        println!(
            "│ {}{} │",
            s,
            " ".repeat(part1_len + part2_len + time_len + 12 - s.len()),
        );
    }
    {
        println!("└{}┘", "─".repeat(part1_len + part2_len + time_len + 14));
    }
}

/// LOADING FUNCS
///
///
///
///
///
/// LOADING FUNCS

fn load<P: AsRef<Path> + Debug + Copy>(path: P) -> String {
    #[allow(clippy::expect_fun_call)]
    fs::read_to_string(path).expect(&format!("Failed to find file {:?}!", path))
}

pub fn load_to_matrix<A, B, C>(path: A) -> IntoIter<IntoIter<char>>
where
    A: AsRef<Path> + Debug + Copy,
    B: FromIterator<char>,
    C: FromIterator<B>,
{
    load(path)
        .split('\n')
        .map(|r| r.chars().collect::<Vec<_>>().into_iter())
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn load_to_rows<A>(path: A) -> IntoIter<String>
where
    A: AsRef<Path> + Debug + Copy,
{
    load(path)
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn load_to_rows_and_pattern<A, P>(path: A, p: P) -> IntoIter<IntoIter<String>>
where
    A: AsRef<Path> + Debug + Copy,
    P: Fn(char) -> bool + Copy,
{
    load(path)
        .split('\n')
        .map(|s| {
            s.split(p)
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn load_to_pattern<A, P>(path: A, p: P) -> IntoIter<String>
where
    A: AsRef<Path> + Debug + Copy,
    P: Fn(char) -> bool + Copy,
{
    load(path)
        .split(|c| c == '\n' || p(c))
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .into_iter()
}
