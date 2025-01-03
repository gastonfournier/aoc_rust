use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

#[cfg(test)]
const SAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn middle(numbers: &[i32]) -> i32 {
    let numbers = numbers.to_vec();
    numbers[numbers.len() / 2]
}

#[test]
fn middle_test() {
    assert_eq!(middle(&[75,47,61,53,29]), 61);
    assert_eq!(middle(&[97,61,53,29,13]), 53);
    assert_eq!(middle(&[75,29,13]), 29);
}

fn sum_middles(updates: Vec<Vec<i32>>) -> i32 {
    updates.iter().map(|numbers| middle(numbers)).sum()
}

#[test]
fn sum_middles_test() {
    assert_eq!(sum_middles(vec![
        vec![75,47,61,53,29],
        vec![97,61,53,29,13],
        vec![75,29,13],
    ]), 143);
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {

    let mut rules: Vec<(i32, i32)> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];
    let mut found_new_line = false;
    input.lines().for_each(|line| {
        if line.is_empty() {
            found_new_line = true;
        } else if !found_new_line {
            let mut p = line.split('|').map(|s| s.trim().parse::<i32>().unwrap());
            rules.push((p.next().unwrap(), p.next().unwrap()));
        } else if found_new_line {
            updates.push(line.split(',').map(|s| s.parse::<i32>().unwrap()).collect());
        }
    });
    
    (rules, updates)
}

#[aoc(day5, part1)]
fn solve_part1(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let (rules, updates) = input;
    // find fast if any pair does not fulfill rules
    let fail_rules = move |a: i32, b: i32| rules.iter().any(|(x, y)| a == *y && b == *x);
    let correct_ones: Vec<Vec<i32>> = updates.iter().filter(|update| {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if fail_rules(update[i], update[j]) {
                    return false;
                }
            }
        }
        return true;
    }).cloned().collect();
    sum_middles(correct_ones)
}

#[test]
fn test_sample_part1() {
    let (rules, updates) = parse_input(SAMPLE_INPUT);
    assert_eq!(solve_part1(&(rules, updates)), 143);
}

#[aoc(day5, part2)]
fn solve_part2(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let (rules, updates) = input;
    // find fast if any pair does not fulfill rules
    let fail_rules = move |a: i32, b: i32| rules.iter().any(|(x, y)| a == *y && b == *x);
    let incorrect_ones: Vec<Vec<i32>> = updates.iter().filter(|update| {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if fail_rules(update[i], update[j]) {
                    return true;
                }
            }
        }
        return false;
    }).cloned().collect();

    let sorted: Vec<Vec<i32>> = incorrect_ones.iter().map(|l| {
        let mut nl = l.clone();
        nl.sort_by(|a, b| if !fail_rules(*a , *b) { Ordering::Less } else { Ordering::Greater });
        nl
    }).collect();
    sum_middles(sorted)
}

#[test]
fn test_sample_part2() {
    let (rules, updates) = parse_input(SAMPLE_INPUT);
    assert_eq!(solve_part2(&(rules, updates)), 123);
}