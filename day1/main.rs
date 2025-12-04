use std::fs;

fn main() {
    let input = fs::read_to_string("day1/input.txt").expect("input.txt not found");
    let lines = input.trim().lines();
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;

    for line in lines {
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        let step: i32 = if direction == 'L' { -1 } else { 1 };

        for _ in 0..distance {
            position = (position + step).rem_euclid(100);

            if position == 0 {
                zero_count += 1;
            }
        }
    }

    println!("{}", zero_count);
}