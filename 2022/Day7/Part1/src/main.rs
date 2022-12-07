use std::fs;
use std::path::PathBuf;

fn calculate_space_to_be_removed(input: String) -> usize {
    let lines = input.lines();
    let file_sizes: Vec<usize> = vec![]; 
    let mut current_path = PathBuf::new();
    
    for line in lines {
        let chars: Vec<&str> = line.split(' ').into_iter().collect();
        if chars[0] == "$" {
            if chars[1] != "ls" {
                if chars[2] == ".." {
                    // we remove the last path
                    current_path.pop();
                } else {
                    current_path.push(chars[2]);
                }   
            }
        } 

        
        if !["$", "dir"].contains(&chars[0]) {
            let file_size = chars[0].parse::<usize>();
            

        }
    }
    
    file_sizes.into_iter().filter(|x| x < &100000).sum::<usize>()
}

fn main() {
    let file_path = "./input.txt".to_string();

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{:?}", calculate_space_to_be_removed(contents))
}

#[cfg(test)]
mod tests {
    use crate::calculate_space_to_be_removed;
    use std::fs;

    #[test]
    fn test_all_cases() {
        let file_path = "./test_data.txt".to_string();

        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        assert_eq!(calculate_space_to_be_removed(contents), 95437);
    }
}
