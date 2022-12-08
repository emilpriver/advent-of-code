use std::fs;

fn get_vertical_trees(input: String, index: usize) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            (line.chars().nth(index).unwrap().to_string())
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>()
}

fn count_visible_tree(input: String) -> usize {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let total_lines = lines.len();

    // we want to add the vertical edge lines total trees to the variable
    let mut total_visible_trees = total_lines * 2;

    for (index, line) in lines.into_iter().enumerate() {
        if index == 0 || index == (total_lines - 1) {
            total_visible_trees += line.len();
            // all of these trees are already visible
            continue;
        }

        let mut visible_trees = 0;
        let line_to_chars: Vec<i64> = line
            .chars()
            .map(|x| x.to_string().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let line_as_iter = line_to_chars.clone().into_iter().enumerate();

        for (char_index, char) in line_as_iter {
            if char_index == 0 || char_index == (line_to_chars.len() - 1) {
                // the first char and the last is already calculated
                continue;
            }

            let horizontal_split = line_to_chars.split_at(char_index);
            if horizontal_split.0.iter().all(|x| x < &char)
                || horizontal_split.1.iter().all(|x| x < &char)
            {
                // tree is visible, lets add it
                visible_trees += 1;
            } else {
                let vertical_tree = get_vertical_trees(input.clone(), char_index);
                let vertical_split = line_to_chars.split_at(char_index);
                if vertical_split.0.iter().all(|x| x < &char)
                    || vertical_split.1.iter().all(|x| x < &char)
                {
                    visible_trees += 1;
                }
            }
        }

        total_visible_trees += visible_trees
    }

    total_visible_trees
}

fn main() {
    let file_path = "./input.txt".to_string();

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{:?}", count_visible_tree(contents))
}

#[cfg(test)]
mod tests {
    use crate::count_visible_tree;
    use std::fs;

    #[test]
    fn test_all_cases() {
        let file_path = "./test_data.txt".to_string();

        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        assert_eq!(count_visible_tree(contents), 21);
    }
}
