use std::cmp::Ordering;
use std::env;
use std::fs;

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_array = contents.lines();

    let mut equal_chars: Vec<char> = vec![];

    for line in contents_array {
        let (first_part, last_part) = line.split_at(line.len() / 2);
        let mut first_part_array = first_part.chars().into_iter();
        let last_part_array: Vec<char> = last_part.chars().collect();

        loop {
            match first_part_array.next() {
                Some(x) => {
                    if last_part_array.contains(&x) {
                        equal_chars.push(x);
                    }
                },
                None => break,
            }
        }
    }

    println!("{:?}", equal_chars)
}
