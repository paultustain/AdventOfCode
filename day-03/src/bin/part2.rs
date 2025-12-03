fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

fn find_largest(reduced_vec: &[u32]) -> Option<(usize, u32)> {
    let biggest_value = reduced_vec.iter().max().unwrap();
    for (n, v) in reduced_vec.iter().enumerate() {
        if v == biggest_value {
            return Some((n + 1, *v));
        }
    }
    None
}

const TOTAL_USED: usize = 12;

fn get_largest(bank_vec: Vec<u32>) -> u64 {
    let mut idx: usize = 0;
    let mut batteries_used: Vec<String> = Vec::new();

    for n in 1..=TOTAL_USED {
        let (add_idx, value) =
            find_largest(&bank_vec[idx..bank_vec.len() - (TOTAL_USED - n)]).unwrap();
        idx += add_idx;
        batteries_used.push(value.to_string());
    }

    batteries_used.join("").parse::<u64>().unwrap()
}

fn find_output(input: &str) -> String {
    let banks: Vec<&str> = input.split("\n").collect();
    let mut count: u64 = 0;

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

        assert_eq!(result, "3121910778619".to_string());
    }

    #[test]
    fn test_one() {
        let result = find_output("987654321111111");

        assert_eq!(result, "987654321111".to_string());
    }

    #[test]
    fn test_two() {
        let result = find_output("811111111111119");

        assert_eq!(result, "811111111119".to_string());
    }
    #[test]
    fn test_three() {
        let result = find_output("234234234234278");

        assert_eq!(result, "434234234278".to_string());
    }

    #[test]
    fn test_four() {
        let result = find_output("818181911112111");

        assert_eq!(result, "888911112111".to_string());
    }

    #[test]
    fn test_largest_idx() {
        let (idx, value) = find_largest(&Vec::from([9, 6, 5, 3, 7])).unwrap();
        assert_eq!((1, 9), (idx, value));
    }

    #[test]
    fn test_multi_largest() {
        let (idx, value) = find_largest(&Vec::from([9, 6, 5, 3, 9, 7, 1, 4, 7, 9])).unwrap();
        assert_eq!((1, 9), (idx, value));
    }
}
