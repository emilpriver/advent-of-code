use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.trim().split('\n').collect();

    let sum: usize = lines
        .chunks(3)
        .map(|chunk| {
            let first_line: HashSet<char> = chunk[0].chars().collect();
            let second_line: HashSet<char> = chunk[1].chars().collect();
            let third_line: HashSet<char> = chunk[2].chars().collect();
            
            [first_line, second_line, third_line] 
        })
        .map(|chunk| {
            let [first_line, second_line, third_line] = chunk; 
            let i1: HashSet<char> = first_line
                .intersection(&second_line)
                .copied()
                .collect();
            let i2: String = i1.intersection(&third_line).take(1).collect();
            i2.chars().next().unwrap()
        })
        .map(|char| {
            let ascii: usize = char as usize;
            if ascii > 96 { ascii - 96 } else { ascii - 38 }
        })
        .sum();

    println!("total points {:?} ", sum)
}
