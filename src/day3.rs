use std::collections::{HashSet, HashMap};
use std::error::Error;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn manhattan((x, y): &(i64, i64)) -> i64 {
    x.abs() + y.abs()
}

fn trace(line: &str) -> Vec<(i64, i64)> {
    let mut curr = (0, 0);
    let mut points: Vec<(i64, i64)> = Vec::new();
    for instr in line.split(',') {
        let n = instr[1..].parse::<i64>().unwrap();
        for _ in 1 ..= n {
            match instr.chars().nth(0).unwrap() {
                'U' => curr.1 += 1,
                'D' => curr.1 -= 1,
                'R' => curr.0 += 1,
                'L' => curr.0 -= 1,
                x => panic!("unknown direction: {}", x),
            }
            if curr != (0, 0) {
                points.push(curr);
            }
        }
    }
    return points;
}

fn p1(line1: &str, line2: &str) -> Option<i64> {
    let pts1: HashSet<(i64, i64)> = HashSet::from_iter(trace(line1));
    let pts2: HashSet<(i64, i64)> = HashSet::from_iter(trace(line2));
    pts1.intersection(&pts2)
        .map(|p| manhattan(p))
        .min()
}

fn p2(line1: &str, line2: &str) -> Option<usize> {
    let path1 = trace(line1);
    let path2 = trace(line2);
    let mut t: HashMap<(i64, i64), (usize, usize)> = HashMap::new();

    for (i, p) in path1.into_iter().enumerate() {
        let a = t.entry(p).or_insert((0,0));
        a.0 = i+1;
    }

    for (i, p) in path2.into_iter().enumerate() {
        let a = t.entry(p).or_insert((0,0));
        a.1 = i+1;
    }

    t
        .into_iter()
        .filter_map( |(_, (a, b))| {
            if a > 0 && b > 0 {
                Some(a+b)
            } else {
                None
            }
        })
        .min()
}

#[test]
fn test_p1() {
    assert_eq!(Some(159),
               p1("R75,D30,R83,U83,L12,D49,R71,U7,L72",
                  "U62,R66,U55,R34,D71,R55,D58,R83"));
    assert_eq!(Some(135),
               p1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                  "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
}

#[test]
fn test_p2() {
    assert_eq!(Some(610),
               p2("R75,D30,R83,U83,L12,D49,R71,U7,L72",
                  "U62,R66,U55,R34,D71,R55,D58,R83"));
    assert_eq!(Some(410),
               p2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                  "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let lines: Vec<String> = stdin
        .lines()
        .filter_map(|line| line.ok())
        .take(2)
        .collect();
    let nearest = p1(&lines[0], &lines[1]);
    let shortest = p2(&lines[0], &lines[1]);
    println!("{:?} {:?}", nearest, shortest);

    return Ok(());
}
