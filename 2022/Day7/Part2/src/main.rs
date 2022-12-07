use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn calculate_space(input: String) -> usize {
    let lines = input.lines();
    let mut file_sizes: HashMap<String, usize> = HashMap::new();
    let mut current_path = PathBuf::new();

    for line in lines {
        let chars: Vec<&str> = line.split(' ').into_iter().collect();
        if chars[0] == "$" && chars[1] != "ls" {
            if chars[2] == ".." {
                // we remove the last path
                current_path.pop();
            } else {
                current_path.push(chars[2]);
            }
        }

        if !["$", "dir"].contains(&chars[0]) {
            let file_size = chars[0].parse::<usize>().unwrap();
            let mut path = current_path.clone();

            loop {
                *file_sizes
                    .entry(path.to_string_lossy().to_string())
                    .or_default() += file_size;
                if !path.pop() {
                    break;
                };
            }
        }
    }

    *file_sizes
        .values()
        .filter(|x| x > &&(30_000_000 - (70_000_000 - file_sizes["/"])))
        .min()
        .unwrap()
}

fn main() {
    let file_path = "./input.txt".to_string();

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{:?}", calculate_space(contents))
}

#[cfg(test)]
mod tests {
    use crate::calculate_space;
    use std::fs;

    #[test]
    fn test_all_cases() {
        let file_path = "./test_data.txt".to_string();

        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        assert_eq!(calculate_space(contents), 24933642);
    }
}
