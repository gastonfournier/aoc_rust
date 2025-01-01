
use aoc_runner_derive::{aoc, aoc_generator};

#[cfg(test)]
const SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

const NEEDLE: &str = "XMAS";

#[derive(Debug, Clone, Copy)]
enum Direction {
    Top, TopRight, Right, BottomRight, Bottom, BottomLeft, Left, TopLeft
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Top,
            Direction::TopRight,
            Direction::Right,
            Direction::BottomRight,
            Direction::Bottom,
            Direction::BottomLeft,
            Direction::Left,
            Direction::TopLeft
        ].into_iter()
    }
}

#[test]
fn sample_part_1() {
    let mults = parse_input(SAMPLE_INPUT);
    println!("{:#?}", &mults);
    assert_eq!(solve_part1(&mults), 18);
}

#[test]
fn sample_part_2() {
    let mults = parse_input(SAMPLE_INPUT);
    assert_eq!(solve_part2(&mults), 9);
}

type Graph = Vec<Vec<char>>;

fn char_at(input: &Graph, x: usize, y: usize) -> Option<char> {
    if y >= input.len() {
        return None;
    }
    if x >= input[y].len() {
        return None;
    }
    Some(input[y][x])
}

fn next(x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
    let (x, y) = match dir {
        Direction::Top => (Some(x), y.checked_sub(1)),
        Direction::TopRight => (Some(x + 1), y.checked_sub(1)),
        Direction::Right => (Some(x + 1), Some(y)),
        Direction::BottomRight => (Some(x + 1), Some(y + 1)),
        Direction::Bottom => (Some(x), Some(y + 1)),
        Direction::BottomLeft => (x.checked_sub(1), Some(y + 1)),
        Direction::Left => (x.checked_sub(1), Some(y)),
        Direction::TopLeft => (x.checked_sub(1), y.checked_sub(1)),
    };
    match (x, y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None
    }
}

fn find(needle: &str, input: &Graph, x: usize, y: usize, d: Direction) -> bool {
    match needle.chars().next() {
        Some(c) => {
            let current = char_at(input, x, y);
            match current {
                Some(current) => {
                    if current != c {
                        false
                    } else {
                        match next(x, y, d) {
                            Some((x, y)) => {
                                find(&needle[1..], input, x, y, d)
                            }
                            None => needle.len() == 1
                        }
                    }
                }
                None => false
            }
        }
        None => true
    }
}

#[test]
fn find_test() {
    let input = vec![vec!['a', 'b'], vec!['c', 'd']];
    assert!(!find("ab", &input, 0, 0, Direction::Top));
    assert!(!find("ab", &input, 0, 0, Direction::TopRight));
    assert!(find("ab", &input, 0, 0, Direction::Right));
    assert!(!find("ab", &input, 0, 0, Direction::BottomRight));
    assert!(!find("ab", &input, 0, 0, Direction::Bottom));
    assert!(!find("ab", &input, 0, 0, Direction::BottomLeft));
    assert!(!find("ab", &input, 0, 0, Direction::Left));
    assert!(!find("ab", &input, 0, 0, Direction::TopLeft));
    
    assert!(find("ca", &input, 0, 1, Direction::Top));
    assert!(find("cb", &input, 0, 1, Direction::TopRight));
    assert!(find("ad", &input, 0, 0, Direction::BottomRight));
    assert!(find("ac", &input, 0, 0, Direction::Bottom));
    assert!(find("bc", &input, 1, 0, Direction::BottomLeft));
    assert!(find("ba", &input, 1, 0, Direction::Left));
    assert!(find("da", &input, 1, 1, Direction::TopLeft));
    
}
fn xmas(input: &Graph, x: usize, y: usize) -> usize {
    Direction::iter().map(|d| find(NEEDLE, input, x, y, d)).filter(|b| *b).count()
}

fn x_mas(input: &Graph, x: usize, y: usize) -> usize {
    if (find("MAS", input, x+1, y+1, Direction::TopLeft) || find("SAM", input, x+1, y+1, Direction::TopLeft)) && 
        y.checked_sub(1).map(|y| 
            find("MAS", input, x+1, y, Direction::BottomLeft) || find("SAM", input, x+1, y, Direction::BottomLeft)
        ).unwrap_or(false) {
        1
    } else {
        0
    }
}

#[test]
fn next_underflow() {
    assert_eq!(next(0, 0, Direction::Top), None);
    assert_eq!(next(0, 0, Direction::TopLeft), None);
    assert_eq!(next(0, 0, Direction::Left), None);
    assert_eq!(next(0, 0, Direction::BottomLeft), None);
}

#[test]
fn next_overflow() {
    assert_eq!(next(1, 1, Direction::Top), Some((1, 0)));
    assert_eq!(next(1, 1, Direction::TopRight), None);
    assert_eq!(next(1, 1, Direction::Right), None);
    assert_eq!(next(1, 1, Direction::BottomRight), None);
    assert_eq!(next(1, 1, Direction::Bottom), None);
    assert_eq!(next(1, 1, Direction::BottomLeft), None);
    assert_eq!(next(1, 1, Direction::Left), Some((0, 1)));
    assert_eq!(next(1, 1, Direction::TopLeft), Some((0, 0)));
}

#[test]
fn char_at_outside_bounds() {
    let input = vec![vec!['a', 'b'], vec!['c', 'd']];
    assert_eq!(char_at(&input, 0, 2), None);
    assert_eq!(char_at(&input, 1, 1), Some('d'));
    assert_eq!(char_at(&input, 1, 0), Some('b'));
}

#[test]
fn parse_input_test() {
    let input = vec![vec!['a', 'b'], vec!['c', 'd']];
    assert_eq!(parse_input("ab
cd"), input);
}

#[aoc_generator(day4)]
fn parse_input(input: &str) ->  Vec<Vec<char>> {
    input.lines().map(|line| {
        line.chars().collect()
    }).collect()
}


#[aoc(day4, part1)]
fn solve_part1(graph: &Graph) -> i32 {
    let mut sum = 0;
    for y in 0..graph.len() {
        for x in 0..graph[y].len() {
            sum += xmas(graph, x, y) as i32;
        }
    }
    sum
}

#[aoc(day4, part2)]
fn solve_part2(graph: &Graph) -> i32 {
    let mut sum = 0;
    for y in 0..graph.len() {
        for x in 0..graph[y].len() {
            sum += x_mas(graph, x, y) as i32;
        }
    }
    sum
}