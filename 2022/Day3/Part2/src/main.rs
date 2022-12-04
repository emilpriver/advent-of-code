use std::fs;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut equal_chars: Vec<char> = vec![];
    let mut total_points: usize = 0;

    let lines: Vec<&str> = contents.trim().split('\n').collect();

    lines.chunks(3).map(|chunk| {
        let first_line: HashSet<char> = chunk[0].chars().collect();
        let second_line: HashSet<char> = chunk[1].chars().collect();
        let third_line: HashSet<char> = chunk[2].chars().collect();

        let first_part_array: Vec<char> =
            first_line.into_iter().unique().collect();
        let second_part_array: Vec<char> =
            second_line.into_iter().unique().collect();
        let mut third_part_array: Vec<char> =
            third_line.into_iter().unique().collect();

        for x in &first_part_array {
            for y in &second_part_array {
                if third_part_array.contains(x) && third_part_array.contains(y) {
                    equal_chars.push(*x);

                    // we need to remove the char from the array
                    third_part_array
                        .remove(third_part_array.iter().position(|&r| r == *x).unwrap());
                }
            }
        }
    });

    for char in equal_chars {
        let ascii: usize = char as usize;
        total_points += if ascii > 96 { ascii - 96 } else { ascii - 38 }
    }

    println!("total points {:?} ", total_points)
}
