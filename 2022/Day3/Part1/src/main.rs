use itertools::Itertools;
use std::fs;

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_array = contents.lines();

    let mut equal_chars: Vec<char> = vec![];
    let mut total_points: usize = 0;

    for line in contents_array {
        let (first_part, last_part) = line.split_at(line.len() / 2);
        let first_part_array = first_part.chars();
        let mut last_part_array: Vec<char> = last_part.chars().into_iter().unique().collect();

        for x in first_part_array {
            if last_part_array.contains(&x) {
                equal_chars.push(x);

                // we need to remove the char from the array
                last_part_array.remove(last_part_array.iter().position(|&r| r == x).unwrap());
            }
        }
    }

    for char in equal_chars {
        let ascii: usize = char as usize;
        total_points += if ascii > 96 {
            ascii - 96
        } else {
            ascii - 38
        }
    }

    println!("total points {:?}", total_points)
}
