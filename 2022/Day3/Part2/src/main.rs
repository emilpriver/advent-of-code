use itertools::Itertools;
use std::fs;

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_array = contents.lines().chunks(3);

    let mut equal_chars: Vec<char> = vec![];
    let mut total_points: usize = 0;

    for line in &contents_array {
        let line_into_vec = line.collect_vec();
        let [first_line, second_line, third_line] = line_into_vec[0..2];
        let first_part_array: Vec<char> = first_line.chars().into_iter().unique().collect();
        let second_part_array: Vec<char> = second_line.chars().into_iter().unique().collect();
        let third_part_array: Vec<char> = third_line.chars().into_iter().unique().collect();

        for x in first_part_array {
            for y in second_part_array {
                if third_part_array.contains(&x) && third_part_array.contains(&y) {
                    equal_chars.push(x);

                    // we need to remove the char from the array
                    third_part_array.remove(third_part_array.iter().position(|&r| r == x).unwrap());
                }
            }
        }
    }

    for char in equal_chars {
        let ascii: usize = char as usize;
        total_points += if ascii > 96 { ascii - 96 } else { ascii - 38 }
    }

    println!("total points {:?} ", total_points)
}
