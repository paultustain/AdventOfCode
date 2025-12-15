fn main() {
    let input = include_str!("./input.txt");
    let output = find_output(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Edge {
    start: Position,
    end: Position,
}

impl Position {
    fn get_size(&self, other: &Position) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

fn get_edges(points: &Vec<Position>) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for idx in 0..points.len() - 1 {
        edges.push(Edge {
            start: points[idx],
            end: points[idx + 1],
        })
    }
    edges.push(Edge {
        start: points[points.len() - 1],
        end: points[0],
    });
    edges
}

fn get_squares(input: &Vec<Position>) -> Vec<Vec<Position>> {
    let mut squares: Vec<Vec<Position>> = Vec::new();
    for (n, position1) in input.iter().enumerate() {
        for m in n + 1..input.len() {
            let position2 = input[m];
            let mut points: Vec<Position> = Vec::new();

            points.push(Position {
                x: position1.x,
                y: position1.y,
            });
            points.push(Position {
                x: position2.x,
                y: position2.y,
            });

            squares.push(points);
        }
    }

    squares
}

fn square_contained(square: &Vec<Position>, green_edge: &Vec<Edge>) -> bool {
    let mut x_coord: Vec<i64> = square.iter().map(|point| point.x).collect::<Vec<i64>>();
    x_coord.dedup();
    if x_coord.len() == 1 {
        return true;
    }
    let mut y_coord: Vec<i64> = square.iter().map(|point| point.y).collect::<Vec<i64>>();
    y_coord.dedup();
    if y_coord.len() == 1 {
        return true;
    }
    let mut result = true;
    for line in green_edge {
        let left = square[0].x.max(square[1].x) <= line.start.x.min(line.end.x);
        let right = square[0].x.min(square[1].x) >= line.start.x.max(line.end.x);
        let above = square[0].y.max(square[1].y) <= line.start.y.min(line.end.y);
        let below = square[0].y.min(square[1].y) >= line.start.y.max(line.end.y);

        result = left || right || above || below;
    }

    result
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
                x: values[0].parse::<i64>().unwrap(),
                y: values[1].parse::<i64>().unwrap(),
            }
        })
        .collect();

    let squares = get_squares(&positions);
    let green_edge = get_edges(&positions);
    let mut max = 0;
    for square in squares {
        if square_contained(&square, &green_edge) {
            if square[0].get_size(&square[1]) > max {
                max = square[0].get_size(&square[1]);
            }
        }
    }
    max.to_string()
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
        assert_eq!("24".to_string(), result);
    }
}
