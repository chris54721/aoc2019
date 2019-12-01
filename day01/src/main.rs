use std::fs;

fn main() {
    let input: Vec<i64> = fs::read_to_string("./day01/input/input.txt").unwrap()
        .lines().map(|x| x.parse().unwrap()).collect();
    let fuel: Vec<_> = input.iter().map(|m| (m / 3) - 2).collect();
    println!("Part 1: {}", &fuel.iter().sum::<i64>());
    println!("Part 2: {}", &fuel.iter().map(total_fuel).sum::<i64>());
}

fn total_fuel(fuel: &i64) -> i64 {
    let f = (fuel / 3) - 2;
    fuel + (if f > 0 { total_fuel(&f) } else { 0 })
}