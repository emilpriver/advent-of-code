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

                structure[index].push(el.trim().to_string());
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
            .filter(|x| !x.is_empty() && x.chars().all(|x| x.is_ascii_digit()))
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();

        let amount = instructions[0];
        let from = instructions[1];
        let to = instructions[2];

        for _ in 0..amount {
            if let Some(x) = structure[from - 1].pop() {
                structure[to - 1].push(x);
            }
        }
    }

    let mut first_row: Vec<String> = vec![];
    for c in &structure {
        first_row.push(c.get(0).unwrap().to_string())
    }

    let first_row_as_word = first_row
        .join("")
        .replace("[", "")
        .replace("]", "")
        .chars()
        .filter(|c| c.is_ascii())
        .collect::<String>();
    println!("{:?}", first_row_as_word)
}
