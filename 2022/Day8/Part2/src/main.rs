use std::fs;

fn get_vertical_trees(input: String, vert_index: usize) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            (line.chars().nth(vert_index).unwrap().to_string())
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>()
}

fn count_visible_tree(input: String) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let total_lines = lines.len();

    // we want to add the vertical edge lines total trees to the variable
    let mut total_visible_trees: i64 = (total_lines * 2).to_string().parse::<i64>().unwrap();

    for (vert_index, line) in lines.into_iter().enumerate() {
        if vert_index == 0 || vert_index == (total_lines - 1) {
            // add all the trees in the top and bottom line
            total_visible_trees += line.len().to_string().parse::<i64>().unwrap();
            continue;
        }

        let line_to_chars: Vec<i64> = line
            .chars()
            .map(|x| x.to_string().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let line_as_iter = line_to_chars.clone().into_iter().enumerate();

        for (hor_index, char) in line_as_iter {
            let char_as_int = char.to_string().parse::<i64>().unwrap();

            if hor_index == 0 || hor_index == (line_to_chars.len() - 1) {
                // the first and last line is already visible
                continue;
            }
            let horizontal_split = line_to_chars.split_at(hor_index);
            if horizontal_split.0.iter().all(|x| x < &char_as_int)
                || horizontal_split.1[1..].iter().all(|x| x < &char_as_int)
            {
                // tree is visible, lets add it
                total_visible_trees += 1;
        
                continue;
            }

            let vertical_tree = get_vertical_trees(input.clone(), hor_index);
            let vertical_split = vertical_tree.split_at(vert_index);
            if vertical_split.0.iter().all(|x| x < &char_as_int)
                || vertical_split.1[1..].iter().all(|x| x < &char_as_int)
            {
                // tree is visible, lets add it

                total_visible_trees += 1;
            }
        }
    }

    total_visible_trees - 4
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
        
        let file_path = "./input.txt".to_string();

        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        assert_eq!(count_visible_tree(contents), 1733);
    }
}
