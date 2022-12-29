use std::io;
use std::fs::File;
use std::collections::VecDeque;

pub fn main(mut lines: io::Lines<io::BufReader<File>>) {
    let line_result = lines.next().unwrap();
    let line = line_result.unwrap();


    let width = 14;

    let mut seen: [i32; 26] = [0; 26];

    let mut i = 0;
    let mut distinct = 0;

    let mut window: VecDeque<char> = VecDeque::new();

    for ch in line.chars() {
        /*
        println!("i: {}, Window: {:?}, Distinct: {}", i, window, distinct);
        println!("Seen: {:?}", seen);
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
        */

        i += 1;
        window.push_back(ch);
        let val = (ch as usize) - ('a' as usize);
        if seen[val] == 0 {
            distinct += 1;
        }
        seen[val] += 1;

        if window.len() > width {
            let discard = window.pop_front().unwrap();
            let discard_val = (discard as usize) - ('a' as usize);
            if seen[discard_val] == 1 {
                distinct -= 1;
            }
            seen[discard_val] -= 1;
        }

        if distinct == width {
            break;
        }
    }

    println!("Found {} distinct codon after {} chars", width, i);
}