fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    col: usize,
}

#[derive(Debug, Eq, PartialOrd, Ord)]
struct Beam {
    position: Position,
    number: u64,
}

impl PartialEq for Beam {
    fn eq(&self, other: &Beam) -> bool {
        self.position == other.position
    }
}

fn get_start(row: &Vec<&str>) -> Option<Position> {
    for (n, chr) in row.iter().enumerate() {
        if *chr == "S" {
            return Some(Position { col: n });
        }
    }

    None
}

fn find_output(input: &str) -> String {
    let input_vector: Vec<&str> = input.trim().split('\n').collect();
    let mut map: Vec<Vec<&str>> = Vec::new();
    for row in input_vector {
        let new_row: Vec<&str> = row.split("").collect::<Vec<&str>>();
        map.push(new_row[1..(new_row.len() - 1)].to_vec());
    }

    let mut beams: Vec<Beam> = Vec::new();
    let mut splitter_hit = 0;
    for row in map {
        let mut updated_beams: Vec<Beam> = Vec::new();
        if beams.len() == 0 {
            let start = get_start(&row);
            beams.push(Beam {
                position: start.unwrap(),
                number: 1,
            });
        } else {
            for beam in &beams {
                let next_item = row[beam.position.col];
                if next_item == "." {
                    let add_beam = Beam {
                        position: Position {
                            col: beam.position.col,
                        },
                        number: beam.number,
                    };
                    if !updated_beams.contains(&add_beam) {
                        updated_beams.push(add_beam);
                    } else {
                        for update_beam in updated_beams.iter_mut() {
                            if update_beam.position == add_beam.position {
                                update_beam.number += add_beam.number;
                            }
                        }
                    }
                } else if next_item == "^" {
                    let add_beam = Beam {
                        position: Position {
                            col: beam.position.col - 1,
                        },
                        number: beam.number,
                    };
                    if !updated_beams.contains(&add_beam) {
                        updated_beams.push(add_beam);
                    } else {
                        for update_beam in updated_beams.iter_mut() {
                            if update_beam.position == add_beam.position {
                                update_beam.number += add_beam.number;
                            }
                        }
                    }
                    let add_beam = Beam {
                        position: Position {
                            col: beam.position.col + 1,
                        },
                        number: beam.number,
                    };
                    if !updated_beams.contains(&add_beam) {
                        updated_beams.push(add_beam);
                    } else {
                        for update_beam in updated_beams.iter_mut() {
                            if update_beam.position == add_beam.position {
                                update_beam.number += add_beam.number;
                            }
                        }
                    }
                }
            }

            beams = updated_beams;
        }
    }

    for beam in beams {
        splitter_hit += beam.number;
    }

    splitter_hit.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!("40".to_string(), result);
    }
}
