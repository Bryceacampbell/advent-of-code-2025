use std::fs;

fn is_invalid(id: i64) -> bool {
    let id_string = id.to_string();
    let id_length = id_string.len();

    for pattern_length in 1..=id_length / 2 {
        if id_length % pattern_length != 0 {
            continue;
        }

        let pattern = &id_string[..pattern_length];
        let repeats = id_length / pattern_length;

        if pattern.repeat(repeats) == id_string {
            return true;
        }
    }
    false
}

fn main() {
    let input = fs::read_to_string("day2/input.txt").expect("input.txt not found");
    let lines: String = input.trim().lines().collect();
    let ranges: Vec<&str> = lines.split(',').collect();
    let mut invalid_count: i64 = 0;

    for range in ranges {
        let range_parts: Vec<&str> = range.split('-').collect();
        let range_start: i64 = range_parts[0].parse().unwrap();
        let range_end: i64 = range_parts[1].parse().unwrap();

        for i in range_start..=range_end {
            if is_invalid(i) {
                invalid_count += i;
            }
        }
    }

    println!("Invalid Count: {}", invalid_count);
}
