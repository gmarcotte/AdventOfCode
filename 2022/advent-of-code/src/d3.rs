use std::io;
use std::fs::File;

pub fn priority(ch: char) -> Result<usize, String> { 
    let val = ch as usize;
    if val >= ('a' as usize) && val <= ('z' as usize) {
        Ok(val - ('a' as usize))
    } else if val >= ('A' as usize) && val <= ('Z' as usize) {
        Ok(val - ('A' as usize) + 26)
    } else {
        Err(format!("invalid character: {}", ch))
    }
}

fn find_duplicate_item(pack: &String) -> usize {
    let num = pack.chars().count();
    let mut seen: [i32; 52] = [0; 52];
    let mut i = 0;
    for ch in pack.chars() {
        let pri = priority(ch).unwrap();
        if i < num / 2 {
            seen[pri] += 1;
        } else {
            if seen[pri] > 0 {
                return pri + 1;
            }
        }
        i += 1;
    }
    return 0;
}

fn find_shared_item(packs: &Vec<String>) -> usize {
    let mut seen: [i32; 52] = [0; 52];
    for i in 0..packs.len() {
        for ch in packs[i].chars() {
            let pri = priority(ch).unwrap();
            seen[pri] |= 1 << i;
        }
    }

    let max_seen = (1 << packs.len()) - 1;
    for i in 0..52 {
        if seen[i] == max_seen {
            return i + 1;
        }
    }
    return 0;
}

pub fn main(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    /*
    for line in lines {
        if let Ok(pack) = line {
            total += find_duplicate_item(&pack);
        }
    }
    */

    let chunk_size = 3;
    let mut chunk: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(pack) = line {
            chunk.push(pack);
            if chunk.len() == chunk_size {
                let pri = find_shared_item(&chunk);
                total += pri;
                chunk = Vec::new();
            }
        }
    }

    println!("Total {}", total);
}