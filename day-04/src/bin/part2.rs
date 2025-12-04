use core::fmt;

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Item {
    item: char,
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.item)
    }
}

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

fn clear_rolls(map: &mut Vec<Vec<Item>>, to_remove: Vec<Position>) {
    for position in to_remove {
        map[position.row][position.col] = Item { item: '.' };
    }
}

fn find_surrounding(position: &Position, map_size: &MapSize, map: &Vec<Vec<Item>>) -> bool {
    if map[position.row][position.col] != (Item { item: '@' }) {
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
                if map[(position.row as i32 + x) as usize][(position.col as i32 + y) as usize]
                    == (Item { item: '@' })
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
    let mut rows: Vec<&str> = input.split("\n").collect();
    let mut movable = 0;

    let size = MapSize {
        width: rows[0].len(),
        height: rows.len() - 1,
    };

    let mut map: Vec<Vec<Item>> = Vec::new();

    for row in rows.iter() {
        let mut row_vec: Vec<Item> = Vec::new();
        if row.len() > 0 {
            for chr in row.chars() {
                row_vec.push(Item { item: chr });
            }
            map.push(row_vec);
        }
    }

    let mut start_map: Vec<Vec<Item>> = Vec::new();

    while start_map != map {
        start_map = map.clone();
        let mut to_remove: Vec<Position> = Vec::new();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, _) in row.chars().enumerate() {
                let position = Position {
                    col: col_idx,
                    row: row_idx,
                };

                if find_surrounding(&position, &size, &map) {
                    to_remove.push(position);
                    movable += 1;
                }
            }
        }
        clear_rolls(&mut map, to_remove);
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

        assert_eq!("43", result);
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
