use std::error::Error;
use std::io::{self, BufRead};

fn p1(mass: i64) -> i64 {
    return (mass / 3) - 2;
}

#[test]
fn test_part1() {
    assert_eq!(p1(12), 2);
    assert_eq!(p1(14), 2);
    assert_eq!(p1(1969), 654);
    assert_eq!(p1(100756), 33583);
}

fn p2(mut mass: i64) -> i64 {
    let mut total_fuel = 0;
    loop {
        let fuel = p1(mass);
        if fuel <= 0 {
            break;
        }
        total_fuel += fuel;
        mass = fuel;
    }
    return total_fuel;
}

#[test]
fn test_part_2() {
    assert_eq!(p2(14), 2);
    assert_eq!(p2(1969), 966);
    assert_eq!(p2(100756), 50346);
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut total_fuel_1 = 0;
    let mut total_fuel_2 = 0;
    for line in stdin.lines() {
        let line = line?;
        let mass = line.parse::<i64>()?;
        total_fuel_1 += p1(mass);
        total_fuel_2 += p2(mass);
    }
    println!("{} {}", total_fuel_1, total_fuel_2);
    return Ok(());
}
