fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

#[derive(Debug)]
struct Position {
    x: i128,
    y: i128,
}

impl Position {
    fn get_size(&self, other: &Position) -> i128 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

fn get_pairs(input: Vec<Position>) -> Vec<i128> {
    let mut sizes: Vec<i128> = Vec::new();
    for (n, position1) in input.iter().enumerate() {
        for m in n + 1..input.len() {
            sizes.push(position1.get_size(&input[m]));
        }
    }
    sizes
}

fn find_output(input: &str) -> String {
    let positions: Vec<Position> = input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let values = line.split(",").collect::<Vec<&str>>();
            Position {
                x: values[0].parse::<i128>().unwrap(),
                y: values[1].parse::<i128>().unwrap(),
            }
        })
        .collect();

    let pairs = get_pairs(positions);
    pairs.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!("50".to_string(), result);
    }
}
