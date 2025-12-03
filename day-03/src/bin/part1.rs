fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

fn find_largest(reduced_vec: &[u32]) -> Option<(usize, u32)> {
    let biggest_value = reduced_vec.iter().max().unwrap();
    for (n, v) in reduced_vec.iter().enumerate() {
        if v == biggest_value {
            return Some((n, *v));
        }
    }
    None
}

fn get_largest(bank_vec: Vec<u32>) -> u32 {
    let idx: usize = 0;
    let mut batteries_used: Vec<String> = Vec::new();

    let (idx, value) = find_largest(&bank_vec[idx..bank_vec.len() - 1]).unwrap();
    batteries_used.push(value.to_string());

    let (_, value) = find_largest(&bank_vec[idx + 1..bank_vec.len()]).unwrap();
    batteries_used.push(value.to_string());

    batteries_used.join("").parse::<u32>().unwrap()
}

fn find_output(input: &str) -> String {
    let banks: Vec<&str> = input.split("\n").collect();
    let mut count: u32 = 0;

    for bank in banks {
        if bank == "" {
            break;
        }
        let bank_vec: Vec<u32> = bank.chars().flat_map(|ch| ch.to_digit(10)).collect();
        count += get_largest(bank_vec);
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

    #[test]
    fn test_largest_idx() {
        let (idx, value) = find_largest(&Vec::from([9, 6, 5, 3, 7])).unwrap();
        assert_eq!((0, 9), (idx, value));
    }

    #[test]
    fn test_multi_largest() {
        let (idx, value) = find_largest(&Vec::from([9, 6, 5, 3, 9, 7, 1, 4, 7, 9])).unwrap();
        assert_eq!((0, 9), (idx, value));
    }
}
