fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u128,
    end: u128,
}

fn non_unique_ranges(current_ranges: &mut Vec<Range>, to_add: Range) {
    if current_ranges.len() == 0 {
        current_ranges.push(to_add);
        return;
    }
    let final_range = current_ranges.pop().unwrap();
    if final_range.end < to_add.start {
        current_ranges.push(final_range);
        current_ranges.push(to_add);
        return;
    } else if final_range.end >= to_add.start {
        let new_end = final_range.end.max(to_add.end);

        let new_range = Range {
            start: final_range.start,
            end: new_end,
        };
        current_ranges.push(new_range);
        return;
    }
}

fn get_ranges(input: Vec<&str>) -> (Vec<Range>, Vec<&str>) {
    let mut fresh_range: Vec<Range> = Vec::new();
    let mut vec_break: usize = 0;

    for range in &input {
        if *range == "" {
            break;
        }
        let range_split: Vec<&str> = range.split("-").collect();
        let start = match range_split[0].parse::<u128>() {
            Ok(value) => value,
            Err(error) => panic!("Problem with range start: {error:?}"),
        };
        let end = match range_split[1].parse::<u128>() {
            Ok(value) => value,
            Err(error) => panic!("Problem with range end: {error:?}"),
        };

        let rng = Range {
            start: start,
            end: end,
        };

        fresh_range.push(rng);

        vec_break += 1;
    }

    fresh_range.sort_unstable();

    return (fresh_range, input[(vec_break + 1)..].to_vec());
}

fn find_output(input: &str) -> String {
    let input_vec: Vec<&str> = input.trim().split('\n').collect();
    let (fresh_range, _) = get_ranges(input_vec);

    let mut fresh_count: u128 = 0;
    let mut clean_ranges: Vec<Range> = Vec::new();

    dbg!(&fresh_range);

    for rng in fresh_range {
        non_unique_ranges(&mut clean_ranges, rng);
    }

    dbg!(&clean_ranges);

    for rng in clean_ranges {
        fresh_count += (rng.end - rng.start) + 1;
    }

    fresh_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32
",
        );
        assert_eq!("14".to_string(), result);
    }

    #[test]
    fn test_two() {
        let result = find_output(
            "3-5
10-20
11-14

1",
        );
        assert_eq!("14".to_string(), result);
    }

    #[test]
    fn test_three() {
        let result = find_output(
            "154677529022353-159180259958574

1",
        );

        assert_eq!("4502730936222".to_string(), result);
    }
}
