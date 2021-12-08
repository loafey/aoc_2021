use aoc_lib::load_to_rows_and_pattern;

/*
      0
    ddddd
 1 e     a 2
   e  3  a
    fffff
 4 g     b 5
   g     b
    ccccc
      6
*/
#[rustfmt::skip]
fn parse_7_digit_num(s: &str, pat: [char;7]) -> char {
    let mut num = [false; 7];
    if s.contains(pat[0]) { num[0] = true; }
    if s.contains(pat[1]) { num[1] = true; }
    if s.contains(pat[2]) { num[2] = true; }
    if s.contains(pat[3]) { num[3] = true; }
    if s.contains(pat[4]) { num[4] = true; }
    if s.contains(pat[5]) { num[5] = true; }
    if s.contains(pat[6]) { num[6] = true; }

    match num {
        [true,  true,   true, false,  true,  true,  true] => '0',
        [false, false,  true, false, false,  true, false] => '1',
        [true,  false,  true,  true,  true, false,  true] => '2',
        [true,  false,  true,  true, false,  true,  true] => '3',
        [false,  true,  true,  true, false,  true, false] => '4',
        [true,   true, false,  true, false,  true,  true] => '5',
        [true,   true, false,  true,  true,  true,  true] => '6',
        [true,  false,  true, false, false,  true, false] => '7',
        [true,   true,  true,  true,  true,  true,  true] => '8',
        [true,   true,  true,  true, false,  true,  true] => '9',
        _ => '-'
    }
}

pub fn part1() -> i32 {
    let mut unique = 0;
    load_to_rows_and_pattern("input/day8.txt", |c| c == '|').for_each(|mut r| {
        r.next();
        let output = r.next().unwrap();

        output
            .split(' ')
            .filter(|s| !s.is_empty())
            .for_each(|s| match s.len() {
                1 | 2 | 3 | 4 | 7 | 8 => unique += 1,
                _ => {}
            });
    });

    unique
}

pub fn part2() -> i32 {
    load_to_rows_and_pattern("input/day8.txt", |c| c == '|')
        .map(|mut r| {
            let input = r.next().unwrap();
            let output = r.next().unwrap();

            let mut pat = [""; 10];
            let splat = input.split(' ').filter(|s| !s.is_empty());
            splat.clone().for_each(|f| {
                match f.len() {
                    2 => pat[1] = f, // 1
                    4 => pat[4] = f, // 4
                    3 => pat[7] = f, // 7
                    7 => pat[8] = f, // 8
                    _ => {}
                }
            });

            let a = pat[7].chars().find(|c| !pat[1].contains(*c)).unwrap();
            {
                let p = pat;
                splat.clone().filter(|s| !p.contains(s)).for_each(|s| {
                    if s.chars().filter(|ch| pat[4].contains(*ch)).count() == 4 {
                        pat[9] = s;
                    }
                });
            }
            let g = pat[9]
                .chars()
                .find(|ch| !pat[4].contains(*ch) && *ch != a)
                .unwrap();
            let e = pat[8]
                .chars()
                .find(|ch| !pat[4].contains(*ch) && *ch != a && *ch != g)
                .unwrap();
            {
                let p = pat;
                splat.clone().filter(|s| !p.contains(s)).for_each(|s| {
                    if s.chars()
                        .filter(|ch| *ch != a && *ch != g && *ch != e)
                        .count()
                        == 2
                    {
                        pat[2] = s;
                    }
                });
            }
            {
                let p = pat;
                splat.clone().filter(|s| !p.contains(s)).for_each(|s| {
                    if s.chars().filter(|ch| !pat[2].contains(*ch)).count() == 1 {
                        pat[3] = s;
                    }
                });
            }
            let c = pat[1].chars().find(|ch| pat[2].contains(*ch)).unwrap();
            let f = pat[1].chars().find(|ch| !pat[2].contains(*ch)).unwrap();
            let d = pat[3]
                .chars()
                .find(|ch| ![a, c, e, f, g].contains(ch))
                .unwrap();
            let b = pat[8]
                .chars()
                .find(|ch| ![a, c, e, f, g, d].contains(ch))
                .unwrap();

            let input = [a, b, c, d, e, f, g];

            let mut output_num = String::new();
            output
                .split(' ')
                .filter(|s| !s.is_empty())
                .for_each(|s| output_num.push(parse_7_digit_num(s, input)));

            output_num.parse::<i32>().unwrap()
        })
        .sum::<i32>()
}
