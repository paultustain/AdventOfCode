fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn find_sum(inputs: Vec<i32>) -> i32 {
    let mut zero: i32 = 0;
    let mut dial: i32 = 50;

    for dial_move in inputs {
        let full_dial = dial + dial_move;
        zero += (full_dial / 100).abs();
        if dial != 0 && full_dial <= 0 {
            zero += 1;
        }

        dial = full_dial.rem_euclid(100);
    }
    zero
}

fn part2(input: &str) -> String {
    let mut input = str::replace(input, "L", "-");
    input = str::replace(&input, "R", "");
    let vec_input: Vec<i32> = input
        .split("\n")
        .map(|val| val.parse::<i32>().unwrap_or(0))
        .collect();

    find_sum(vec_input).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let result = part2(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );

        assert_eq!(result, "6".to_string());
    }
}
