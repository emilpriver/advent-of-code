use std::fs;

fn get_vertical_trees(input: String, vert_index: usize) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            (line.chars().nth(vert_index).unwrap().to_string())
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>()
}

fn count_lower_trees(slice: Vec<usize>, tree_height: usize, reverse: bool) -> usize {
    let mut score = 0;
    let mut tree_slice = slice.clone();

    if reverse {
        tree_slice.reverse()
    }

    for tree in &tree_slice {
        score += 1;

        if tree_height <= *tree {
            break;
        }
    }

    score
}

fn find_heighest_tree(input: String) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let total_lines = lines.len();

    // we want to add the vertical edge lines total trees to the variable
    let mut best_tree_score = 0;

    for (vert_index, line) in lines.into_iter().enumerate() {
        if vert_index == 0 || vert_index == (total_lines - 1) {
            // add all the trees in the top and bottom line
            continue;
        }

        let line_to_chars: Vec<usize> = line
            .chars()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let line_as_iter = line_to_chars.clone().into_iter().enumerate();

        for (hor_index, char) in line_as_iter {
            let char_as_int = char.to_string().parse::<usize>().unwrap();

            if hor_index == 0 || hor_index == (line_to_chars.len() - 1) {
                // the first and last line is already visible
                continue;
            }

            let horizontal_split = line_to_chars.split_at(hor_index);
            let vertical_tree = get_vertical_trees(input.clone(), hor_index);
            let vertical_split = vertical_tree.split_at(vert_index);

            let top_score: usize = count_lower_trees(vertical_split.0.to_vec(), char_as_int, true);
            let bottom_score: usize =
                count_lower_trees(vertical_split.1[1..].to_vec(), char_as_int, false);
            let left_score: usize =
                count_lower_trees(horizontal_split.0.to_vec(), char_as_int, true);
            let right_score: usize =
                count_lower_trees(horizontal_split.1[1..].to_vec(), char_as_int, false);
            let result = top_score * left_score * bottom_score * right_score;

            if result > best_tree_score {
                best_tree_score = result;
            }
        }
    }

    best_tree_score
}

fn main() {
    let file_path = "./input.txt".to_string();

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{:?}", find_heighest_tree(contents))
}

#[cfg(test)]
mod tests {
    use crate::find_heighest_tree;
    use std::fs;

    #[test]
    fn test_all_cases() {
        let file_path = "./test_data.txt".to_string();

        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        assert_eq!(find_heighest_tree(contents), 8);
    }
}
