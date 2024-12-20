use aoc_runner_derive::{aoc, aoc_generator};

#[cfg(test)]
const SAMPLE_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";


#[test]
fn sample_part_1() {
    let mults = parse_input(SAMPLE_INPUT);
    println!("{:#?}", &mults);
    assert_eq!(solve_part1(&mults), 161);
}

#[test]
fn sample_part_2() {
    let muls = parse_input_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    println!("{:#?}", &muls);
    assert_eq!(solve_part2(&muls), 48);
}


#[aoc_generator(day3, part2)]
fn parse_input_2(input_v: &str) -> Vec<(i32,i32)> {
    let mul_state = vec!["m", "u", "l", "(", "[0-9]+", ",", "[0-9]+", ")"];
    let do_state = vec!["d", "o", "(", ")"];
    let dont_state = vec!["d", "o", "n", "'", "t", "(", ")"];
    let mut current_state: Option<Vec<&str>> = None;
    let mut muls: Vec<(i32, i32)> = Vec::new();
    let mut at = 0;
    let mut a: Option<i32> = None;
    let mut b: Option<i32> = None;
    let mut ignoring = false;
    for c in input_v.chars() {
        if current_state.is_none() {
            let current = c.to_string();
            if current == mul_state[0] {
                current_state = Some(mul_state.clone());
            } else if current == do_state[0] {
                current_state = Some(do_state.clone());
            }
            at = 0;
            a = None;
            b = None;
        }
        if current_state.is_some() {
            let pat = current_state.clone().unwrap()[at];
            println!("[{}] Matching {} against {} vs {:?}", at, c, pat, current_state.clone().unwrap());
            if pat == "[0-9]+" && c.is_numeric() {
                if let Some(n) = c.to_digit(10) {
                    if at == 4 {
                        a = Some(a.unwrap_or(0) * 10 + n as i32);
                    } else {
                        b = Some(b.unwrap_or(0) * 10 + n as i32);
                    }
                }
                if (a.unwrap_or(0) > 999) || (b.unwrap_or(0) > 999) {
                    println!("Reset: invalid number {:?} or {:?}", a, b);
                    at = 0;
                    a = None;
                    b = None;
                    current_state = None;
                }
            } else if pat == "[0-9]+" && !c.is_numeric() {
                println!("Checking {} against {:?} or {:?}", c, a, b);
                if a.is_some() && b.is_none() && c == ',' {
                    at += 2; // skip comma state
                    println!("Found comma moving on");
                } else if b.is_some() && c == ')' {
                    println!("   (ignoring={}) Found mult ({:?}*{:?})", ignoring, a, b);
                    if !ignoring {
                        muls.push((a.unwrap(), b.unwrap()));
                    }
                    at = 0;
                    a = None;
                    b = None;
                    current_state = None;
                } else {
                    at = 0;
                    a = None;
                    b = None;
                    current_state = None;
                }
            } else if current_state.clone().unwrap() == mul_state && pat == c.to_string() {
                at += 1;
            } else if current_state.clone().unwrap() == do_state && pat == c.to_string() {
                if at == 2 {
                    at = 0;
                    a = None;
                    b = None;
                    current_state = None;
                    ignoring = false;
                } else {
                    at += 1;
                }
            } else if current_state.clone().unwrap() != mul_state && dont_state[at] == c.to_string() {
                if at == 5 {
                    at = 0;
                    a = None;
                    b = None;
                    current_state = None;
                    ignoring = true;
                } else {
                    at += 1;
                    current_state = Some(dont_state.clone());
                }
            } else {
                at = 0;
                a = None;
                b = None;
                current_state = None;
            }
        }
    }
    
    muls
}


#[aoc_generator(day3, part1)]
fn parse_input(input_v: &str) -> Vec<(i32,i32)> {
    let state = vec!["m", "u", "l", "(", "[0-9]+", ",", "[0-9]+", ")"];
    let mut muls: Vec<(i32, i32)> = Vec::new();
    let mut at = 0;
    let mut a: Option<i32> = None;
    let mut b: Option<i32> = None;
    for c in input_v.chars() {
        let pat = state[at];
        println!("[{}] Matching {} against {}", at, c, pat);
        if pat == "[0-9]+" && c.is_numeric() {
            if let Some(n) = c.to_digit(10) {
                if at == 4 {
                    a = Some(a.unwrap_or(0) * 10 + n as i32);
                } else {
                    b = Some(b.unwrap_or(0) * 10 + n as i32);
                }
            }
            if (a.unwrap_or(0) > 999) || (b.unwrap_or(0) > 999) {
                println!("Reset: invalid number {:?} or {:?}", a, b);
                at = 0;
                a = None;
                b = None;
            }
        } else if pat == "[0-9]+" && !c.is_numeric() {
            println!("Checking {} against {:?} or {:?}", c, a, b);
            if a.is_some() && b.is_none() && c == ',' {
                at += 2; // skip comma state
                println!("Found comma moving on");
            } else if b.is_some() && c == ')' {
                println!("    Found mult ({:?}*{:?})", a, b);
                muls.push((a.unwrap(), b.unwrap()));
                at = 0;
                a = None;
                b = None;
            } else {
                at = 0;
                a = None;
                b = None;
            }
        } else if pat == c.to_string() {
            at += 1;
        } else {
            at = 0;
            a = None;
            b = None;
        }
    }
    
    muls
}


#[aoc(day3, part1)]
fn solve_part1(mults: &Vec<(i32, i32)>) -> i32 {
    mults.into_iter().fold(0, |acc, (a,b)| acc + a * b)
}

#[aoc(day3, part2)]
fn solve_part2(mults: &Vec<(i32, i32)>) -> i32 {
    mults.into_iter().fold(0, |acc, (a,b)| acc + a * b)
}

