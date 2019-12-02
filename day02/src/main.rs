use std::fs;

fn main() {
    let input: Vec<usize> = fs::read_to_string("./day02/input/input.txt").unwrap()
        .split(',').map(|x| x.parse().unwrap()).collect();
    println!("Part 1: {}", run(input.clone(), (12, 2)));
    println!("Part 2: {}", part2(&input));
}

fn run(mut tape: Vec<usize>, inputs: (usize, usize)) -> usize {
    tape[1] = inputs.0;
    tape[2] = inputs.1;
    let mut i = 0;
    loop {
        match tape[i] {
            1 => {
                let out = tape[i + 3];
                tape[out] = tape[tape[i + 2]] + tape[tape[i + 1]]
            }
            2 => {
                let out = tape[i + 3];
                tape[out] = tape[tape[i + 2]] * tape[tape[i + 1]]
            }
            99 => break tape[0],
            _ => unreachable!()
        };
        i += 4;
    }
}

fn part2(input: &Vec<usize>) -> usize {
    for i1 in 0..=99 {
        for i2 in 0..=99 {
            if run(input.clone(), (i1, i2)) == 19690720 {
                return 100 * i1 + i2;
            }
        }
    }
    unreachable!("No solution found for part 2");
}