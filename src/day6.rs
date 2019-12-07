use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};

fn parse_line(line: &str) -> Vec<String> {
    line.split(')')
        .map(|x| x.to_string())
        .collect()
}

fn p1(map: &HashMap<String, Option<String>>) -> usize {
    let mut checksum = 0;
    for key in map.keys() {
        checksum += dist_to_root(&map, key.to_string());
    }
    return checksum;
}

fn dist_to_root(map: &HashMap<String, Option<String>>, mut key: String) -> usize {
    let mut dist = 0;
    loop {
        let parent = map.get(&key).expect("cannot get parent");
        match parent {
            None => break,
            Some(new_key) => {
                dist += 1;
                key = new_key.to_string();
            }
        }
    }
    return dist;
}

fn all_parents(map: &HashMap<String, Option<String>>, mut key: String) -> Vec<String> {
    let mut parents = vec![];
    loop {
        let parent = map.get(&key).expect("cannot get parent");
        match parent {
            None => break,
            Some(new_key) => {
                parents.push(new_key.to_string());
                key = new_key.to_string();
            }
        }
    }
    parents.reverse();
    return parents;
}

fn p2(map: &HashMap<String, Option<String>>) -> usize {
    let you_ancestry = all_parents(&map, "YOU".to_string());
    let san_ancestry = all_parents(&map, "SAN".to_string());

    let mut i = 0;
    while you_ancestry[i] == san_ancestry[i] {
        i += 1;
    }
    return (you_ancestry.len() - i) + (san_ancestry.len() - i);
}

#[test]
fn test_p2() {
    let lines = vec![
        "COM)B".to_string(),
        "B)C".to_string(),
        "C)D".to_string(),
        "D)E".to_string(),
        "E)F".to_string(),
        "B)G".to_string(),
        "G)H".to_string(),
        "D)I".to_string(),
        "E)J".to_string(),
        "J)K".to_string(),
        "K)L".to_string(),
        "K)YOU".to_string(),
        "I)SAN".to_string(),
    ];
    let map = make_map(&lines);
    assert_eq!(4, p2(&map));
}

fn make_map(lines: &[String]) -> HashMap<String, Option<String>> {
    let mut map: HashMap<String, Option<String>> = HashMap::new();
    for line in lines {
        let bodies = parse_line(&line);
        let unmoving = bodies[0].clone();
        let moving = bodies[1].clone();

        let _ = map.entry(unmoving.clone()).or_insert(None);
        let x = map.entry(moving).or_insert(None);
        *x = Some(unmoving);
    }
    return map;
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines: Vec<String> = stdin.lines().filter_map(|x| x.ok()).collect();
    let map = make_map(&lines);

    let checksum = p1(&map);
    let dist = p2(&map);
    println!("{} {}", checksum, dist);

    return Ok(());
}
