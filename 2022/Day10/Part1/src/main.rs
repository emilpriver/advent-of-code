use std::fs;

fn julmust(input: String) -> i64 {
    let lines = input.lines();
    let score_index: [i64; 6] = [20, 60, 100, 140, 180, 220];

    let scores = lines.fold(vec![1], |mut instruct, value| {
        instruct.push(instruct[instruct.len() - 1]);
        
        if let Some((a, b)) = value.split_once(' ') {
            if !a.contains("noop") {
                instruct.push(instruct.last().unwrap() + b.parse::<i64>().unwrap());
            };
        };

        instruct
    });

    let values = scores
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, _)| {
            let index_as_int = i as i64;
            if score_index.contains(&(index_as_int)) {
                return index_as_int * scores[i - 1];
            };

            0
        });
    
    values.sum()
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
