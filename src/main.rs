use std::fs;

fn main() {
    let max = 99;
    let min = 0;
    let modulus: i32 = max - min + 1;
    let mut position = 50;
    let mut zero_left_counter: u32 = 0;

    let input = fs::read_to_string("./input/1.txt").unwrap();

    input.lines().enumerate().for_each(|(_, line)| {
        let str_len = line.len();
        let direction = &line[0..1];
        let distance = &line[1..str_len].parse::<i32>().unwrap();
        // l = - , r = +
        if direction == "R" {
            position = (position + distance %modulus) %modulus;
        } else {
            position = (position - distance + modulus) %modulus;
        }

        println!("The dial is rotated {} to point at {}.", line, position);
        if position == min {
            zero_left_counter += 1;
        }
    });
    println!("position: {}, Zero clicks: {}", position, zero_left_counter);
}
