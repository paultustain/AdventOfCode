fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    end: u64,
}

fn get_ranges(input: Vec<&str>) -> (Vec<Range>, Vec<&str>) {
    let mut fresh_range: Vec<Range> = Vec::new();
    let mut vec_break: usize = 0;

    for range in &input {
        if *range == "" {
            break;
        }
        let range_split: Vec<&str> = range.split("-").collect();
        let start = match range_split[0].parse::<u64>() {
            Ok(value) => value,
            Err(error) => panic!("Problem with range start: {error:?}"),
        };
        let end = match range_split[1].parse::<u64>() {
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
    let (mut fresh_range, food_list) = get_ranges(input_vec);

    let mut fresh_count = 0;

    let mut food_list: Vec<u64> = food_list
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    food_list.sort_unstable();
    for food in food_list.iter().rev() {
        let mut drop = false;
        'range_loop: for rng in fresh_range.iter().rev() {
            if food >= &rng.start && food <= &rng.end {
                fresh_count += 1;
                break 'range_loop;
            }

            if food < &rng.start {
                drop = true;
            }
        }

        if drop {
            fresh_range.pop();
        }
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
        assert_eq!("3".to_string(), result);
    }
    /*
    #[test]
    fn test_grouping() {
        let result = reduce_range_list(Vec::from([Range{start: 10, end: 14}, Range{start: 16, end: 20}, Range{start: 12, end: 18}]));
        assert_eq!([Range{start: 10, end: 20}], result);
    }
    */
}
