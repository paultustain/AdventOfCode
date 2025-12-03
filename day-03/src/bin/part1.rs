fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

fn get_largest(details: &str) -> u64 {
    if details == "" {
        return 0;
    }
    let mut first: u64 = 0;
    let mut second: u64 = 0;

    let final_char = (&details[details.len() - 1..].to_string())
        .parse::<u64>()
        .unwrap();
    let details = &details[..details.len() - 1];

    for v in details.chars() {
        let number = (v.to_string()).parse::<u64>().unwrap();
        if number > first {
            first = number;
            second = 0;
        } else {
            if number > second {
                second = number;
            }
        }
    }

    if final_char > second {
        second = final_char;
    }

    first * 10 + second
}

fn find_output(input: &str) -> String {
    let banks: Vec<&str> = input.split("\n").collect();
    let mut count: u64 = 0;

    for bank in banks {
        count += get_largest(bank);
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );

        assert_eq!(result, "357".to_string());
    }

    #[test]
    fn test_one() {
        let result = find_output("987654321111111");

        assert_eq!(result, "98".to_string());
    }

    #[test]
    fn test_two() {
        let result = find_output("811111111111119");

        assert_eq!(result, "89".to_string());
    }
    #[test]
    fn test_three() {
        let result = find_output("234234234234278");

        assert_eq!(result, "78".to_string());
    }

    #[test]
    fn test_four() {
        let result = find_output("818181911112111");

        assert_eq!(result, "92".to_string());
    }
}
