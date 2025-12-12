use std::{fs};

fn main() {
    let max = 99;
    let min = 0;
    let mut position = 50;
    let mut zero_left_counter: u32 = 0;

    let input = fs::read_to_string("./input/1.txt").unwrap();

    input.lines().enumerate().for_each(|(_, line)| {
        let str_len = line.len();
        let direction = &line[0..1];
        let distance = &line[1..str_len].parse::<isize>().unwrap();
        // l = - , r = +
        for _ in 0..distance.clone() {
            if position == max || position == min {
                if direction == "L" {
                    if position == min {
                        position = max;
                    } else if position == max {
                        position -= 1;
                    }
                }
                if direction == "R" {
                    if position == max {
                        position = min;
                    } else if position == min {
                        position += 1
                    }
                }
            } else {
                if direction == "L" {
                    position -= 1
                }
                if direction == "R" {
                    position += 1;
                }
            }
        }
        println!("The dial is rotated {} to point at {}.", line, position);
        if position == min {
            zero_left_counter += 1;
        }
    });
    println!("position: {}, Zero clicks: {}", position, zero_left_counter);
}
