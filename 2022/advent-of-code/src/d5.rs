use std::io;
use std::fs::File; 
use std::collections::VecDeque;

enum ReadPhase {
    STACKS,
    MOVES,
}

pub fn main(lines: io::Lines<io::BufReader<File>>, num_stacks: usize) {

    let mut phase = ReadPhase::STACKS;

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(VecDeque::new());
    }

    for line in lines {
        //println!("{:?}", stacks);
        //let mut s = String::new();
        //std::io::stdin().read_line(&mut s);
        match phase {
            ReadPhase::STACKS => {
                if let Ok(boxes) = line {
                    let box_chars: Vec<char> = boxes.chars().collect();
                    if box_chars.len() == 0 {
                        phase = ReadPhase::MOVES;
                        continue;
                    } else if *(box_chars.get(1).unwrap()) == '1' {
                        continue;
                    }

                    for i in 0..num_stacks {
                        let ch = box_chars.get(i*4 + 1).unwrap();
                        if *ch != ' ' {
                            stacks[i].push_back(*ch);
                        }
                    }
                }
            },

            ReadPhase::MOVES => {
                if let Ok(move_str) = line {
                    let bits: Vec<&str> = move_str.split(' ').collect();
                    assert_eq!(bits.len(), 6);
                    let num: u32 = bits[1].parse().unwrap();
                    let from: usize = bits[3].parse().unwrap();
                    let to: usize = bits[5].parse().unwrap();
                    //println!("move {} from {} to {}", num, from, to);

                    /*
                    for _ in 0..num {
                        let ch = stacks[from - 1].pop_front().unwrap();
                        stacks[to - 1].push_front(ch);
                    }
                    */

                    let mut tmp: Vec<char> = Vec::new();
                    for _ in 0..num {
                        let ch = stacks[from - 1].pop_front().unwrap();
                        tmp.push(ch);
                    }
                    
                    for _ in 0..num {
                        let ch = tmp.pop().unwrap();
                        stacks[to - 1].push_front(ch);
                    }
                }
            }
        }
    }

    println!("Final top boxes:");
    for mut stack in stacks {
        print!("{}", stack.pop_front().unwrap());
    }
    print!("{}", '\n');
}