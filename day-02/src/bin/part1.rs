fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

fn find_invalid(range: Vec<&str>) -> Vec<i64> {
    let start = range[0].trim().parse::<i64>().unwrap();
    let end: i64 = range[1].trim().parse::<i64>().unwrap();
    let mut invalid: Vec<i64> = Vec::new();

    for value in start..=end {
        let digit_len = value.to_string().len();
        if digit_len % 2 == 0 {
            let half = digit_len / 2;
            if &value.to_string()[..half] == &value.to_string()[half..] {
                invalid.push(value);
            }
        }
    }

    invalid
}

fn find_output(input: &str) -> String {
    let ranges: Vec<Vec<&str>> = input
        .split(",")
        .map(|range| range.split("-").collect())
        .collect();

    let mut count: i64 = 0;

    for range in ranges {
        let invalid_list = find_invalid(range);
        count += invalid_list.iter().sum::<i64>();
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );

        assert_eq!(result, "1227775554".to_string());
    }

    #[test]
    fn test_one() {
        let result = find_output("11-22");
        assert_eq!(result, "33".to_string());
    }

    #[test]
    fn test_two() {
        let result = find_output("95-115");
        assert_eq!(result, "99".to_string());
    }
}
