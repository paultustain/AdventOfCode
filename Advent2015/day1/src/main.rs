use std::fs;

fn main()  {
    let input_string: String = fs::read_to_string("lift_order.txt").expect("Can't read file");

    let up = input_string.chars().filter(|x| x == &'(').count();
    let down = input_string.chars().filter(|x| x == &')').count();
    let floor = up - down; 
    println!("{}", floor);
}
