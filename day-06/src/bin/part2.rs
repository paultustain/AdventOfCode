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

fn create_column_values(rows: &mut Vec<Vec<&str>>) -> Vec<u64> {
    let mut values: Vec<u64> = Vec::new();
    let mut string_value: String = String::new();

    while string_value != " ".repeat(rows.len()) {
        string_value = String::new();
        for row in rows.iter_mut() {
            match row.pop() {
                Some(d) => {
                    if d == "" {
                        string_value += " ";
                    } else {
                        string_value += d
                    }
                }
                None => panic!("{:#?}", row),
            };
        }

        if string_value.trim() != "".to_string() {
            values.push(string_value.trim().parse::<u64>().unwrap());
        }
    }

    values
}

fn find_output(input: &str) -> String {
    let mut input_vec: Vec<&str> = input.trim_end().split('\n').collect();

    let operations = input_vec.pop().unwrap().trim();
    let operation_vec = get_value(operations);

    let mut total = 0;
    let mut calculations: Vec<Vec<&str>> = Vec::new();

    for row in input_vec.iter_mut() {
        let mut updated_row = row.split("").collect::<Vec<&str>>();
        if updated_row.last().unwrap() == &"" {
            updated_row.pop().unwrap();
        }
        calculations.push(updated_row);
    }

    for operation in operation_vec.iter().rev() {
        let numbers_to_use = create_column_values(&mut calculations);
        let value: u64 = match &operation.clone()[..] {
            "*" => numbers_to_use.iter().copied().reduce(|a, b| a * b).unwrap(),
            "+" => numbers_to_use.iter().sum(),
            _ => panic!("not valid"),
        };

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
        assert_eq!("3263827".to_string(), result);
    }
}
