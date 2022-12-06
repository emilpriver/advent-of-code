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
    let mut last_letters: Vec<String> = Vec::new();

    for (index, letter) in input.chars().into_iter().enumerate() {
        last_letters.push(letter.to_string());

        if last_letters.len() > 14 {
            last_letters.remove(0);

            if has_unique_elements(last_letters.clone().into_iter()) {
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
            start_of_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
            19
        );
        assert_eq!(
            start_of_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            23
        );
        assert_eq!(
            start_of_packet_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            23
        );
        assert_eq!(
            start_of_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            29
        );
        assert_eq!(
            start_of_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            26
        );
    }
}
