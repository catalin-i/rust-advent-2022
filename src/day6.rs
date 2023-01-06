use std::collections::HashSet;

fn find_marker(input: &str, marker_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(marker_size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == marker_size)
        .map(|pos| pos + marker_size)
}
pub fn p1() {
    let input = include_str!("../input/day6.txt");
    println!("{}", find_marker(input, 4).unwrap())
}

pub fn p2() {
    let input = include_str!("../input/day6.txt");
    println!("{}", find_marker(input, 14).unwrap())
}
#[cfg(test)]


mod tests {
    use super::find_marker;
    use test_case::test_case;


        #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
        #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
        #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
        #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
        #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
        fn test_find_marker(index: usize, input: &str) {
            assert_eq!(Some(index), find_marker(input, 4));
        }

}
