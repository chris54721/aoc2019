use std::fs;
use std::iter::FromIterator;

const IMG_SIZE: usize = 25 * 6;

fn main() {
    let input = fs::read_to_string("./day08/input/input.txt").unwrap()
        .chars().collect::<Vec<char>>();
    let mut max: (usize, usize) = (usize::max_value(), 0);
    let mut image = ['2'; IMG_SIZE];
    for layer in input.chunks(IMG_SIZE) {
        let zeroes = layer.iter().filter(|&c| *c == '0').count();
        if zeroes < max.0 {
            max.0 = zeroes;
            max.1 = layer.iter().filter(|&c| *c == '1').count()
                * layer.iter().filter(|&c| *c == '2').count();
        }
        for (i, &c) in layer.iter().enumerate() {
            if image[i] == '2' && c != '2' {
                image[i] = c;
            }
        }
    }
    let img_ascii = &image.chunks(25)
        .map(|c| String::from_iter(c.iter()).replace('0', " "))
        .collect::<Vec<String>>()
        .join("\n");
    println!("Part 1: {}", max.1);
    println!("Part 2:\n{}", img_ascii);
}
