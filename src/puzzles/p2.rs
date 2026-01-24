use std::fs;

pub fn main() {
    let max = 99;
    let min = 0;
    let modulus: i32 = max - min + 1;
    let mut position = 50;
    let mut zero_left_counter: u32 = 0;
    let mut rotations = 0;

    let input = fs::read_to_string("./input/1.txt").unwrap();

    input.lines().enumerate().for_each(|(_, line)| {
        let str_len = line.len();
        let direction = &line[0..1];
        let distance = &line[1..str_len].parse::<i32>().unwrap();
        // l = - , r = +
        if direction == "R" {
            rotations += (position + distance) / modulus;
            position = (position + distance % modulus) % modulus;
        } else {
            position = (position - distance + modulus) % modulus;

            rotations += (position - distance + modulus) / modulus;
        }

        if position == min {
            zero_left_counter += 1;
        }
    });
    println!(
        "Final result - position: {}, rotations: {}, Zero crossings: {}",
        position, rotations, zero_left_counter
    );
}
