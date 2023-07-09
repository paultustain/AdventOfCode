use std::fs;

struct Present {
    width: u32,
    height: u32,
    length: u32
}

impl Present {
    fn paper(&self) -> u32 {
        let sizes: Vec<u32> = vec![self.width * self.length, 
        self.width * self.height, 
        self.length * self.height]; 

        2 * sizes.iter().sum::<u32>() + sizes.iter().min().unwrap()
    }
    fn ribbon(&self) -> u32 {
        let perimeters: Vec<u32> = vec![2 * (self.width + self.length), 
        2 * (self.width + self.height), 
        2 * (self.length + self.height)];

        perimeters.iter().min().unwrap() + (self.width * self.length * self.height)
    }
}

fn split_line(item: &str) -> Present {
    let sides:Vec<u32> = item.split("x")
    .map(|x| x.parse::<u32>().unwrap())
    .collect();

    let present = Present {
        width: sides[0],
        height: sides[1], 
        length: sides[2]
    };
    present
}

fn main() {
    let mut paper_used: Vec<u32> = Vec::new();
    let mut ribbon_used: Vec<u32> = Vec::new();
    let input: String = fs::read_to_string("input.txt")
    .expect("Couldn't read file");

    for line in input.lines() {
        let present = split_line(line);
        paper_used.push(present.paper());
        ribbon_used.push(present.ribbon());

    };

    println!("Paper needed: {}", paper_used.iter().sum::<u32>());
    println!("Ribbon needed: {}", ribbon_used.iter().sum::<u32>());
}
