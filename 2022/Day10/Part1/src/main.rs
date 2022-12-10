use std::collections::HashMap;
use std::fs;

fn julmust(input: String) -> i64 {
    let lines = input.lines();
    let score_index: [i64; 5] = [20, 60, 100, 140, 180];
    
    lines.into_iter().enumerate().filter_map(|(i,line)| {
        if x.contains("noop") {
            None
        }

        let (_, b) = line.split_once(" ");

        Some(1)
    }).sum()
}

fn main() {
    let file_path = "./input.txt".to_string();

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{:?}", julmust(contents))
}

#[cfg(test)]
mod tests {
    use crate::julmust;
    use std::fs;

    #[test]
    fn test_all_cases() {
        let file_path = "./test_data.txt".to_string();

        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        assert_eq!(julmust(contents), 13140);
    }
}
