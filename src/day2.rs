use aoc_runner_derive::{aoc, aoc_generator};

#[cfg(test)]
const SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[test]
fn sample_part_1() {
    let reports = parse_input_day2(SAMPLE_INPUT);
    println!("{:#?}", &reports);
    assert_eq!(solve_part1(&reports), 2);
}

#[test]
fn sample_part_2() {
    let reports = parse_input_day2(SAMPLE_INPUT);
    println!("{:#?}", &reports);
    assert_eq!(solve_part2(&reports), 4);
}

#[test]
fn test_samples () {
    let samples = "12 10 11 13 15 18 20";
    let reports = parse_input_day2(samples);
    println!("{:?}", solve_part2(&reports));
}

#[aoc_generator(day2)]
fn parse_input_day2(input_v: &str) -> Vec<Vec<i32>> {
    input_v.lines().map(|line| {
        line.split_whitespace().map(
            |x| x.parse::<i32>().unwrap()
        ).collect()
    }).collect()
}

fn is_safe(report: &Vec<i32>) -> Option<usize> {
    let mut prev = Option::<i32>::None;
    let mut up = Option::<bool>::None;
    for (i, &level) in report.iter().enumerate() {
        if prev.is_some() {
            let dist = prev.unwrap() - level;
            if dist.abs() == 0 || dist.abs() > 3 {
                //println!("{:?} not safe at {}", report, i);
                return Some(i);
            }
            up = up.or(Some(dist < 0));
            if dist < 0 && !up.unwrap() {
                //println!("{:?} not safe at {}", report, i);
                return Some(i);
            } else if dist > 0 && up.unwrap() {
                //println!("{:?} not safe at {}", report, i);
                return Some(i);
            }
        }
        prev = Some(level);
    }
    //println!("{:?} SAFE", report);
    Option::<usize>::None
}

#[aoc(day2, part1)]
fn solve_part1(reports: &[Vec<i32>]) -> i32 {
    let safe : Vec<bool> = reports.iter().map(is_safe).map(|idx| idx.is_none()).collect();
    safe.into_iter().filter(|&x| x).count() as i32
}

#[aoc(day2, part2)]
fn solve_part2(reports: &[Vec<i32>]) -> i32 {
    let safe : Vec<bool> = reports.iter().map(|r|  {
        let safe = is_safe(r);
        if safe.is_some() {
            is_safe(&remove_at_index(r, safe.unwrap() - 1)).is_none()
            || is_safe(&remove_at_index(r, safe.unwrap())).is_none()
            // this last case is in case the first pair goes up and then the second pair goes down,
            // so maybe removing the first element all the sequence goes up
            || (safe.unwrap() as i32 - 2 >= 0 && is_safe(&remove_at_index(r, safe.unwrap() - 2)).is_none())
        } else {
            true
        }
    }).collect();
    safe.into_iter().filter(|&x| x).count() as i32
}

fn remove_at_index(vec: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut result = vec[..index].to_vec(); // Elements before the index
    result.extend_from_slice(&vec[index + 1..]); // Elements after the index
    result
}

#[test]
fn test_remove_at_index() {
    let vec = vec![1, 2, 3, 4, 5];
    let index = 2;
    let result = remove_at_index(&vec, index);
    assert_eq!(result, vec![1, 2, 4, 5]);
}