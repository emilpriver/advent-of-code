use std::fs;

fn julmust(input: String) -> usize {
    let lines = input.lines();

    let scores = lines.fold(vec![1], |mut instruct, value| {
        instruct.push(instruct[instruct.len() - 1]);
        if let Some((a, b)) = value.split_once(' ') {
            if !a.contains("noop") {
                instruct.push(instruct.last().unwrap() + b.parse::<i64>().unwrap());
            };
        };
        instruct
    });

    scores.chunks(40).for_each(|x| {
        for (i, c) in x.iter().enumerate() {
            if (i as i64 - c).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    });

    0
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
        assert_eq!(julmust(contents), 1);
    }
}
