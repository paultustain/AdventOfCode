fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

fn get_value(row: &str) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    let mut value_str: String = String::from("");

    for chr in row.chars() {
        if chr == ' ' {
            if !(value_str == String::from("")) {
                values.push(value_str);
                value_str = String::from("");
            }
        } else {
            value_str.push(chr);
        }
    }
    if !(value_str == String::from("")) {
        values.push(value_str);
    }
    values
}

fn find_output(input: &str) -> String {
    let mut input_vec: Vec<&str> = input.trim().split('\n').collect();

    let operations = input_vec.pop().unwrap().trim();

    let operation_vec = get_value(operations);

    let mut calculations: Vec<Vec<String>> = Vec::new();

    for row in input_vec {
        calculations.push(get_value(row));
    }

    let mut total = 0;
    for idx in 0..operation_vec.len() {
        let operator = &operation_vec[idx].clone()[..];
        let mut value = match operator {
            "*" => 1,
            "+" => 0,
            _ => panic!("not valid"),
        };
        for row in &calculations {
            match operator {
                "*" => value *= row[idx].clone().parse::<u64>().unwrap(),
                "+" => value += row[idx].clone().parse::<u64>().unwrap(),
                _ => panic!("not valid"),
            }
        }
        total += value;
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!("4277556".to_string(), result);
    }
}
