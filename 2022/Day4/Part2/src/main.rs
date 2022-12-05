use std::fs;

fn to_int(value: &str) -> i32 {
    value.parse::<i32>().unwrap()
}

fn main() {
    let file_path = "./input.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let sum: usize = lines
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let elfs: Vec<&str> = line.split(',').collect();
            let first_elf: Vec<&str> = elfs[0].split('-').collect();
            let second_elf: Vec<&str> = elfs[1].split('-').collect();
            
            [first_elf, second_elf]
        })
        .map(|chunk| {
            let [first_elf, second_elf] = chunk;
            let first_elf_range = to_int(first_elf[0])..(to_int(first_elf[1]) + 1);
            let second_elf_range = to_int(second_elf[0])..(to_int(second_elf[1]) + 1);
    
            if first_elf_range.contains(&to_int(second_elf[0]))
                || first_elf_range.contains(&to_int(second_elf[1]))
            {
                return 1;
            }

            if second_elf_range.contains(&to_int(first_elf[0]))
                || second_elf_range.contains(&to_int(first_elf[1]))
            {
                return 1;
            }

            // party didn't overlap
            0
        })
        .sum();

    println!("total assignment pairs overlapping: {:?}", sum)
}
