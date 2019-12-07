use std::fs;
use std::collections::HashMap;

type OrbitMap = HashMap<String, Vec<String>>;

fn main() {
    let input: Vec<Vec<String>> = fs::read_to_string("./day06/input/input.txt").unwrap()
        .lines().map(|l| l.split(')').map(|s| s.to_string()).collect()).collect();
    let mut orbits: OrbitMap = OrbitMap::new();
    for orbit in input {
        orbits.entry(orbit[0].to_string()).or_insert_with(|| Vec::new()).push(orbit[1].to_string());
    }
    let centers: Vec<String> = orbits.keys()
        .filter(|&obj| orbits.values().filter(|&x| x.contains(obj)).count() == 0)
        .cloned().collect();
    println!("Part 1: {}", total_orbits(&orbits, Some(&centers), 0));
    println!("Part 2: {}", count_transfers(&orbits));
}


fn total_orbits(orbits: &OrbitMap, centers: Option<&Vec<String>>, layer: usize) -> usize {
    match centers {
        None => 0,
        Some(cc) => layer * cc.len() + cc.iter()
            .fold(0, |acc, c| acc + total_orbits(&orbits, orbits.get(c), layer + 1))
    }
}

fn count_transfers(orbits: &OrbitMap) -> usize {
    let you_center = get_center_path(&orbits, &"YOU".to_string());
    let san_center = get_center_path(&orbits, &"SAN".to_string());
    let mut i = 0;
    loop {
        if you_center[i] != san_center[i] {
            break;
        }
        i += 1;
    }
    you_center.len() + san_center.len() - 2 * i
}

fn get_center_path(orbits: &OrbitMap, obj: &String) -> Vec<String> {
    let mut path = Vec::new();
    let mut cur = obj;
    loop {
        let parent = orbits.iter().find(|(_, sub)| sub.contains(cur));
        match parent {
            None => break,
            Some((c, _)) => {
                cur = c;
                path.push(c.clone());
            }
        }
    }
    path.reverse();
    path
}