fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

#[derive(Debug)]
struct MapSize {
    width: usize,
    height: usize,
}
#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

fn find_surrounding(position: Position, map_size: &MapSize, rows: &Vec<&str>) -> bool {
    if rows[position.row].chars().nth(position.col).unwrap() != '@' {
        return false;
    }

    let mut start_col: i32 = -1;
    let mut end_col: i32 = 1;
    let mut start_row: i32 = -1;
    let mut end_row: i32 = 1;
    let mut count = 0;

    if position.col == 0 {
        start_col = 0;
    }
    if position.col == map_size.width - 1 {
        end_col = 0;
    }
    if position.row == 0 {
        start_row = 0;
    }
    if position.row == map_size.height - 1 {
        end_row = 0;
    }

    for x in start_row..=end_row {
        for y in start_col..=end_col {
            if !(x == 0 && y == 0) {
                /*
                dbg!(x);
                dbg!(y);
                dbg!(&position);
                dbg!(
                    rows[(position.row as i32 + x) as usize]
                        .chars()
                        .nth((position.col as i32 + y) as usize)
                );*/
                if rows[(position.row as i32 + x) as usize]
                    .chars()
                    .nth((position.col as i32 + y) as usize)
                    .unwrap()
                    == '@'
                {
                    count += 1;
                }
                if count > 3 {
                    return false;
                }
            }
        }
    }
    true
}

fn find_output(input: &str) -> String {
    let rows: Vec<&str> = input.split("\n").collect();
    let size = MapSize {
        width: rows[0].len(),
        height: rows.len() - 1,
    };

    let mut movable = 0;

    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, _) in row.chars().enumerate() {
            let position = Position {
                col: col_idx,
                row: row_idx,
            };

            if find_surrounding(position, &size, &rows) {
                movable += 1;
            }
        }
    }

    movable.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
        );

        assert_eq!("13", result);
    }

    #[test]
    fn test_size() {
        let result = find_output(
            "...
...
...
...",
        );
        assert_eq!("0", result);
    }
}
