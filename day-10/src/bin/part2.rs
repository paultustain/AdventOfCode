use itertools::Itertools;
use std::{
    iter::repeat_n,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input.txt");
    // let input = find_output("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");

    let lights: Vec<u32> = vec![3, 5, 4, 7];
    let buttons: Vec<Vec<u32>> = vec![
        vec![0, 0, 0, 1],
        vec![0, 1, 0, 1],
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 1],
        vec![1, 0, 1, 0],
        vec![1, 1, 0, 0],
    ];

    let i: Vec<u32> = (0..buttons.len() as u32).collect();

    'n_loop: for n in 7..11 {
        // let new = repeat_n(&buttons, n).multi_cartesian_product();
        dbg!(&n);

        let indx = repeat_n(i.clone(), n).multi_cartesian_product();
        for prod in indx {
            let pushes = convert_to_array(&prod, &buttons.len());
            if dot_product(&pushes, &buttons, &lights) {
                dbg!(&n);
                break 'n_loop;
            }
        } /*
        let sums = vec![0; prod[0].len()];
        let summed = prod.clone().into_iter().fold(sums, |mut sums, v| {
        v.into_iter().enumerate().for_each(|(i, x)| sums[i] += x);
        sums
        });
        if summed == lights {
        dbg!(&n);
        break 'n_loop;
        }
        /*let transposed: Vec<Vec<u32>> = (0..prod[0].len())
                        .map(|col| (0..prod.len()).map(|row| prod[row][col]).collect())
                        .collect();
            */
            // dbg!(&prod);
            // dbg!(&transposed);
        }*/
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:#?}", end - start);

    /*
        for prod2 in indx {
            dbg!(&prod2);
        }
    */
    return;

    dbg!(&lights);
    dbg!(&buttons);

    let output = dot_product(&vec![1, 3, 0, 3, 1, 2], &buttons, &lights);
    dbg!(output);

    //let result = find_output(input);
    //dbg!(result);
}

fn dot_product(m1: &Vec<u32>, m2: &Vec<Vec<u32>>, check_result: &Vec<u32>) -> bool {
    let mut result: Vec<u32> = Vec::new();

    for a in 0..m2[0].len() {
        let mut value: u32 = 0;
        for b in 0..m1.len() {
            value += m1[b] * m2[b][a];
        }
        result.push(value);
    }

    return check_result == &result;
}

fn convert_to_array(value: &Vec<u32>, button_len: &usize) -> Vec<u32> {
    let mut button_array = vec![0; *button_len];
    for v in value {
        button_array[*v as usize] += 1;
    }

    button_array
}

fn create_matrices(line: &Vec<&str>) -> Option<u32> {
    let lights = &line[2][0..line[2].len() - 1]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    dbg!(&lights);

    return None;

    /*
        let button_len = lights.len();
        let mut buttons = line[1].trim().split(['(', ')']).collect::<Vec<&str>>();
        buttons.retain(|&x| x != " " && x != "");

        let matrix: Vec<Vec<u32>> = buttons
            .iter()
            .map(|value| {
                let values = value
                    .trim()
                    .split(",")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                convert_to_array(&values, &button_len)
            })
            .collect();

        let new_vec: Vec<u32> = (0..matrix.len() as u32).collect();
        let perms = repeat_n(new_vec.clone().into_iter(), 10)
            .multi_cartesian_product()
            .any(|perm| {
                let array = convert_to_array(&perm, &matrix.len());
                dot_product(&array, &matrix, lights)
            });
        /*for pushes in 1..=button_len {
            let perms = repeat_n(new_vec.clone().into_iter(), pushes)
                .multi_cartesian_product()
                .any(|perm| {
                    let array = convert_to_array(&perm, &matrix.len());
                    dot_product(&array, &matrix, lights)
                });

            if perms {
                return Some(pushes as u32);
            }
        }*/

        return None;
    */
}

fn find_output(input: &str) -> String {
    let input_vec = input
        .trim()
        .split('\n')
        .map(|line| {
            let line_split: Vec<&str> = line.split([']', '{']).collect::<Vec<&str>>();
            line_split
        })
        .collect::<Vec<Vec<&str>>>();

    let pushes = input_vec
        .iter()
        .map(|line| create_matrices(line).unwrap())
        .collect::<Vec<u32>>();
    pushes.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let result = find_output(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );

        assert_eq!("33".to_string(), result);
    }

    #[test]
    fn test_one() {
        let result = find_output("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");

        assert_eq!("10".to_string(), result);
    }

    #[test]
    fn test_array_one() {
        let result = convert_to_array(&vec![3], &4);
        assert_eq!(result, [0, 0, 0, 1]);
    }

    #[test]
    fn test_array_two() {
        let result = convert_to_array(&vec![1, 2], &5);
        assert_eq!(result, [0, 1, 1, 0, 0]);
    }

    #[test]
    fn test_array_multi() {
        let result = convert_to_array(&vec![2, 2], &5);
        assert_eq!(result, [0, 0, 2, 0, 0]);
    }
    #[test]
    fn test_dot() {
        let result = dot_product(
            &vec![1, 2, 3],
            &vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
            &vec![0, 0, 0, 0],
        );
        assert_eq!(result, true);
    }
}
