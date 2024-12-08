use aoc_runner_derive::{aoc, aoc_generator};

#[cfg(test)]
const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

#[test]
pub fn sample_part_1() {
    let (a, b) = parse_input_day1(SAMPLE_INPUT);
    println!("{}", solve_part1(&(a, b)));
}

#[test]
pub fn sample_part_2() {
    
    let (a, b) = parse_input_day1(SAMPLE_INPUT);
    println!("{}", solve_part2(&(a, b)));
}


#[aoc_generator(day1)]
pub fn parse_input_day1(input_v: &str) -> (Vec<i32>, Vec<i32>) {
    input_v.lines().map(|line| {
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        (a, b)
    }).unzip()
}


#[aoc(day1, part1)]
pub fn solve_part1(input_p: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut a, mut b) = input_p.clone();
    a.sort();
    b.sort();
    let mut res = 0;
    for (i, x) in a.iter().enumerate() {
        let y = b[i];
        res += (x - y).abs();
    }
    println!("{:#?}", res);
    res
}

#[aoc(day1, part2)]
pub fn solve_part2(input_p: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (a, b) = input_p.clone();
    let mut res = 0;
    for x in a {
        let count = b.iter().filter(|y| x == **y).count();
        res += x * count as i32;
    }
    println!("{:#?}", res);
    res
}