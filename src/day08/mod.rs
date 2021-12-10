use aoc_lib::load_to_rows_and_pattern;

pub fn part1() -> i32 {
    let mut unique = 0;
    load_to_rows_and_pattern("input/day8.txt", |c| c == '|').for_each(|mut r| {
        r.nth(1)
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .for_each(|s| match s.len() {
                1 | 2 | 3 | 4 | 7 | 8 => unique += 1,
                _ => {}
            });
    });

    unique
}

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

    const HO: bool = true;
    const OH: bool = false;
    // Santa is very jolly this year 🎅
    match num {
        [HO,HO,HO,OH,HO,HO,HO] => '0',
        [OH,OH,HO,OH,OH,HO,OH] => '1',
        [HO,OH,HO,HO,HO,OH,HO] => '2',
        [HO,OH,HO,HO,OH,HO,HO] => '3',
        [OH,HO,HO,HO,OH,HO,OH] => '4',
        [HO,HO,OH,HO,OH,HO,HO] => '5',
        [HO,HO,OH,HO,HO,HO,HO] => '6',
        [HO,OH,HO,OH,OH,HO,OH] => '7',
        [HO,HO,HO,HO,HO,HO,HO] => '8',
        [HO,HO,HO,HO,OH,HO,HO] => '9',
        _ => '-'
    }
}

pub fn char_finder<P>(subject: &str, f: P) -> char
where
    P: FnMut(&char) -> bool,
{
    subject.chars().find(f).unwrap()
}

pub fn word_finder<'l, P>(input: &'l str, pat: &[&str], f: P, size: usize) -> &'l str
where
    P: FnMut(&char) -> bool + Copy,
{
    input
        .split(' ')
        .filter(|s| !s.is_empty())
        .filter(|s| !pat.contains(s))
        .find(|s| s.chars().filter(f).count() == size)
        .unwrap()
}

pub fn part2() -> i32 {
    load_to_rows_and_pattern("input/day8.txt", |c| c == '|')
        .map(|mut r| {
            let i = r.next().unwrap();
            let o = r.next().unwrap();

            let mut p = [""; 10];
            i.split(' ').filter(|s| !s.is_empty()).for_each(|f| {
                match f.len() {
                    2 => p[1] = f, // 1
                    4 => p[4] = f, // 4
                    3 => p[7] = f, // 7
                    7 => p[8] = f, // 8
                    _ => {}
                }
            });

            let a = char_finder(p[7], |ch| !p[1].contains(*ch));
            p[9] = word_finder(&i, &p, |ch| p[4].contains(*ch), 4);
            let g = char_finder(p[9], |ch| !p[4].contains(*ch) && *ch != a);
            let e = char_finder(p[8], |ch| !p[4].contains(*ch) && ![a, g].contains(ch));
            p[2] = word_finder(&i, &p, |ch| ![a, g, e].contains(ch), 2);
            p[3] = word_finder(&i, &p, |ch| !p[2].contains(*ch), 1);
            let c = char_finder(p[1], |ch| p[2].contains(*ch));
            let f = char_finder(p[1], |ch| !p[2].contains(*ch));
            let d = char_finder(p[3], |ch| ![a, c, e, f, g].contains(ch));
            let b = char_finder(p[4], |ch| ![a, c, e, f, g, d].contains(ch));

            let mut output_num = String::new();
            o.split(' ')
                .filter(|s| !s.is_empty())
                .for_each(|s| output_num.push(parse_7_digit_num(s, [a, b, c, d, e, f, g])));

            output_num.parse::<i32>().unwrap()
        })
        .sum::<i32>()
}
