use std::io;
use std::fs::File;

pub fn main(lines: io::Lines<io::BufReader<File>>, sum_top: usize) {
    let mut cals_by_elf = Vec::new();
    let mut curr_cals = 0;
    for line in lines {
        if let Ok(cals_str) = line {
            let cals: u32 = match cals_str.parse() {
                Ok(val) => val,
                Err(_e) => 0,
            };
            if cals > 0 {
                curr_cals += cals;
            } else {
                cals_by_elf.push(curr_cals);
                curr_cals = 0;
            }
        }
    }
    cals_by_elf.sort();
    cals_by_elf.reverse();
    let sum: u32 = (&cals_by_elf[..sum_top]).iter().sum();
    println!("Total Cals by top {} elves: {}", sum_top, sum);
}