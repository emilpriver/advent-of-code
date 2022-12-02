use std::fs;

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_array = contents.lines();

    // yolo
    let mut total_score = 0;

    for line in contents_array {
        let round: Vec<&str> = line.split(" ").collect();

        let opponent_as_string = String::from(round[0]);
        let answer = String::from(round[1]);

        match opponent_as_string.as_str() {
            "A" => {
                if answer == "X" {
                    total_score += 4
                }
                if answer == "Y" {
                    total_score += 8
                }
                if answer == "Z" {
                    total_score += 3
                }
            },
            "B" => {
                if answer == "X" {
                    total_score += 1
                }
                if answer == "Y" {
                    total_score += 5
                }
                if answer == "Z" {
                    total_score += 9
                }
            }
            "C" => {
                if answer == "Y" {
                    total_score += 2
                }
                if answer == "X" {
                    total_score += 7
                }
                if answer == "Z" {
                    total_score += 6
                }
            }
            _ => println!("failed"),
        }
    }

    println!("total score: {:?}", total_score)
}
