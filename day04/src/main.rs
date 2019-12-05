use std::fs;
use std::collections::HashSet;

fn main() {
    let input: Vec<usize> = fs::read_to_string("./day04/input/input.txt").unwrap()
        .split('-').map(|x| x.parse().unwrap()).collect();
    let mut valid1 = 0;
    let mut valid2 = 0;
    for n in input[0]..=input[1] {
        let check = is_valid_password(n.to_string());
        if check.0 {
            valid1 += 1;
        }
        if check.1 {
            valid2 += 1;
        }
    }
    println!("Part 1: {}", valid1);
    println!("Part 2: {}", valid2);
}

fn is_valid_password(password: String) -> (bool, bool) {
    let mut double: HashSet<char> = HashSet::new();
    let mut larger_group: HashSet<char> = HashSet::new();
    let p: Vec<char> = password.chars().collect();
    for i in 1..p.len() {
        if p[i] < p[i - 1] {
            return (false, false);
        }
        if p[i] == p[i - 1] {
            double.insert(p[i]);
            if i > 1 && p[i] == p[i - 2] {
                larger_group.insert(p[i]);
            }
        }
    }
    (double.len() > 0, double.len() > larger_group.len())
}