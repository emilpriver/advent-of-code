use std::fs;

fn to_int(value: char) -> i32 {
    (value.to_string()).parse::<i32>().unwrap()
}

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    
    let sum: usize = lines
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|line| {
            println!("{:?}", line);
            let elfs: Vec<&str> = line.split(",").collect(); 
            let first_elf: Vec<char> = elfs[0].chars().filter(|c| c.is_ascii_digit()).collect();
            let second_elf: Vec<char> = elfs[1].chars().filter(|c| c.is_ascii_digit()).collect();
            
            [first_elf, second_elf] 
        })
        .map(|chunk| {
            let [first_elf, second_elf] = chunk; 
            let first_elf_range = to_int(first_elf[0])..(to_int(first_elf[1]) + 1);
            let second_elf_range = to_int(second_elf[0])..(to_int(second_elf[1]) + 1);

            if first_elf_range.contains(&to_int(second_elf[0])) && first_elf_range.contains(&to_int(second_elf[1])) {
                return 1
            }

            if second_elf_range.contains(&to_int(first_elf[0])) && second_elf_range.contains(&to_int(first_elf[1])) {
                return 1
            }

            // party didn't overlap
            0
        })
        .sum();
    
    println!("total assignment pairs overlapping: {:?}", sum)
}   
