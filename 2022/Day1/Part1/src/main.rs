use std::fs;

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents_array = contents.lines();

    // yolo
    let mut array: [i32; 1000] = [0; 1000];
    let mut current_index = 0;

    let mut highest_calories_index = 0;
    let mut highest_calories = 0;

    for line in contents_array {
        if line.is_empty() {
            current_index += 1;
        } else {
            let string_to_int: i32 = line.parse().unwrap();

            array[current_index] += string_to_int;

            if array[current_index] > highest_calories {
                highest_calories = array[current_index];
                highest_calories_index = current_index;
            }
        }
    }

    println!("highest value of calories: {:?}", highest_calories);
    // offset by 1 to not make the first elf be 0
    println!(
        "highest index of calories: {:?}",
        highest_calories_index + 1
    )
}
