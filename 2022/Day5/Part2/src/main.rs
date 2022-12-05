use std::fs;

fn main() {
    let file_path = "./structure.txt".to_string();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut structure: Vec<Vec<String>> = Vec::new();

    for line in lines {
        if line.contains('[') {
            let elements = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();

            for (index, el) in elements.iter().enumerate() {
                if structure.len() < (index + 1) {
                    structure.push(Vec::new())
                }

                if !el.trim().is_empty() {
                    structure[index].insert(0, el.trim().to_string());
                }
            }
        }
    }

    let file_path = "./input.txt".to_string();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = contents.split('\n').collect();

    for line in input {
        if line.is_empty() {
            continue;
        }

        let instructions: Vec<usize> = line
            .split(' ')
            .filter(|x| !x.trim().is_empty() && x.chars().all(|x| x.is_ascii_digit()))
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();

        let amount = instructions[0].to_string().parse::<usize>().unwrap();
        let from = instructions[1];
        let to = instructions[2];

        if !structure[from - 1].is_empty() {
            let start_position = structure[from - 1].len() - amount;
            let mut v = structure[from - 1].split_off(start_position);
            structure[to - 1].append(&mut v);
        }
    }

    let mut first_row: Vec<String> = vec![];
    for c in &structure {
        let mut char: Vec<&String> = c.iter().filter(|x| !x.is_empty()).collect();
        let value = &char.pop().unwrap();
        first_row.push(value.to_string())
    }

    let first_row_as_word = first_row.join("").replace(['[', ']', ' '], "");
    println!("{:?}", first_row_as_word)
}
