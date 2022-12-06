use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn start_of_packet_marker(input: String) -> usize {
    let mut last_four_letters: Vec<String> = Vec::new();

    for (index, letter) in input.chars().into_iter().enumerate() {
        last_four_letters.push(letter.to_string());

        if last_four_letters.len() > 4 {
            last_four_letters.remove(0);

            if has_unique_elements(last_four_letters.clone().into_iter()) {
                return index + 1;
            }
        }
    }

    0
}

fn main() {
    let file_path = "./input.txt".to_string();

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{:?}", start_of_packet_marker(contents))
}

#[cfg(test)]
mod tests {
    use crate::start_of_packet_marker;

    #[test]
    fn test_all_cases() {
        assert_eq!(
            start_of_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            5
        );
        assert_eq!(
            start_of_packet_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            6
        );
        assert_eq!(
            start_of_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            10
        );
        assert_eq!(
            start_of_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            11
        );
    }
}
