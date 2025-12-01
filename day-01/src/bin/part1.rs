fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn find_sum(inputs: Vec<f32>) -> usize {
    let mut zero: usize = 0;
    for len in 1..=inputs.len() {
        let subset = &inputs[0..len];
        if (subset.iter().sum::<f32>() - 50.) % 100. == 0. {
            zero += 1
        }
    }

    zero
}

fn part1(input: &str) -> String {
    let mut input = str::replace(input, "L", "-");
    input = str::replace(&input, "R", "");
    let vec_input: Vec<f32> = input
        .split("\n")
        .map(|val| val.parse::<f32>().unwrap_or(0.))
        .collect();

    find_sum(vec_input).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let result = part1(
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

        assert_eq!(result, "3".to_string());
    }
}
