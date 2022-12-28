use std::io;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl FromStr for Assignment {
    type Err = std::string::ParseError;
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let bits: Vec<&str> = code.split("-").collect();
        assert_eq!(bits.len(), 2); 

        let start: u32 = bits[0].parse().unwrap();
        let end: u32 = bits[1].parse().unwrap();
        
        Ok(Assignment {start, end})
    }
}

fn fully_overlapping(a: &Assignment, b: &Assignment) -> bool {
    (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
}

fn partially_overlapping(a: &Assignment, b: &Assignment) -> bool {
    (a.start <= b.start && a.end >= b.start) || (b.start <= a.start && b.end >= a.start)
}


pub fn main(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    for line in lines {
        if let Ok(pair) = line {
            let assignments: Vec<&str> = pair.split(",").collect();
            assert_eq!(assignments.len(), 2);

            let assign1 = Assignment::from_str(&assignments[0]).unwrap();
            let assign2 = Assignment::from_str(&assignments[1]).unwrap();

            //if fully_overlapping(&assign1, &assign2) {
            if partially_overlapping(&assign1, &assign2) {
                total += 1;
                println!("Overlap: {:?} and {:?}", assign1, assign2);
            } else {
                println!("No overlap: {:?} and {:?}", assign1, assign2);
            }
        }
    }

    println!("Num of overlapping assignments: {}", total);

}