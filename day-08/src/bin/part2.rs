use std::cmp::Ordering;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Position {
    x: i128,
    y: i128,
    z: i128,
}

impl Position {
    fn find_distance(&self, other: &Position) -> f64 {
        let x_dist = (self.x - other.x).pow(2) as f64;
        let y_dist = (self.y - other.y).pow(2) as f64;
        let z_dist = (self.z - other.z).pow(2) as f64;

        (x_dist + y_dist + z_dist).powf(0.5)
    }
}

#[derive(Debug, PartialEq)]
struct Connection {
    distance: f64,
    position1: Position,
    position2: Position,
}

impl Ord for Connection {
    fn cmp(&self, other: &Connection) -> Ordering {
        if self.distance < other.distance {
            Ordering::Less
        } else if self.distance > other.distance {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Connection) -> Option<Ordering> {
        if self.distance < other.distance {
            Some(Ordering::Less)
        } else if self.distance > other.distance {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for Connection {}

fn create_pairs(boxes: &Vec<Position>) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();
    for (n, box1) in boxes.iter().enumerate() {
        for m in (n + 1)..boxes.len() {
            let distance = box1.find_distance(&boxes[m]);
            let conn = Connection {
                distance: distance,
                position1: *box1,
                position2: boxes[m],
            };

            connections.push(conn);
        }
    }
    connections.sort_by(|a, b| a.cmp(b));

    return connections;
}

fn find_output(input: &str) -> String {
    let boxes: Vec<Position> = input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let values = line.split(",").collect::<Vec<&str>>();
            Position {
                x: values[0].parse::<i128>().unwrap(),
                y: values[1].parse::<i128>().unwrap(),
                z: values[2].parse::<i128>().unwrap(),
            }
        })
        .collect();

    let mut circuits: Vec<Vec<Position>> = Vec::new();
    let mut connections = create_pairs(&boxes);

    connections.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    for connection in connections {
        let box1 = connection.position1;
        let box2 = connection.position2;

        let in_circuit = circuits
            .iter()
            .positions(|circuit| (circuit.contains(&box1)) || (circuit.contains(&box2)))
            .collect::<Vec<usize>>();

        match in_circuit.as_slice() {
            [] => circuits.push(vec![box1, box2]),
            [index] => {
                let circuit = circuits.get_mut(*index).unwrap();
                match (circuit.contains(&box1), circuit.contains(&box2)) {
                    (true, true) => {}
                    (true, false) => {
                        circuit.push(box2);
                        if circuit.len() == boxes.len() {
                            return (box1.x * box2.x).to_string();
                        }
                    }
                    (false, true) => {
                        circuit.push(box1);
                        if circuit.len() == boxes.len() {
                            return (box1.x * box2.x).to_string();
                        }
                    }
                    (false, false) => {}
                }
            }
            [index1, index2] => {
                let a = circuits.remove(*index1.max(index2));
                let b = circuits.remove(*index1.min(index2));
                let new_circuit = a
                    .into_iter()
                    .chain(b.into_iter())
                    .unique()
                    .collect::<Vec<Position>>();
                if new_circuit.len() == 1000 {
                    return (box1.x * box2.x).to_string();
                }

                circuits.push(new_circuit);
            }
            _ => panic!("not empty"),
        }
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut circuit = circuits.pop().unwrap();
    let a = circuit.pop();
    let b = circuit.pop();

    dbg!(&a);
    dbg!(&b);

    (a.unwrap().x * b.unwrap().x).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );
        assert_eq!("25272".to_string(), result);
    }

    #[test]
    fn test_2d() {
        let pos1 = Position { x: -6, y: 8, z: 0 };
        let pos2 = Position { x: -3, y: 9, z: 0 };
        let result = pos1.find_distance(&pos2);
        assert_eq!(result, 3.1622776601683795);
    }
}
