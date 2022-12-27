use std::io;
use std::fs::File;
use std::str::FromStr;

#[derive(Copy,Clone,Debug)]
enum Throw {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(Debug)]
struct MatchOutcome {
    me: Throw,
    opp: Throw,
}

impl FromStr for MatchOutcome {
    type Err = std::string::ParseError;
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let opp: Throw = (match code.chars().nth(0).unwrap() {
            'A' => Ok(Throw::ROCK),
            'B' => Ok(Throw::PAPER),
            'C' => Ok(Throw::SCISSORS),
            _ => Err("unable to parse opponent throw"),
        }).unwrap();

        /*
        let me: Throw = (match code.chars().nth(2).unwrap() {
            'X' => Ok(Throw::ROCK),
            'Y' => Ok(Throw::PAPER),
            'Z' => Ok(Throw::SCISSORS),
            _ => Err("unable to parse my throw")
        }).unwrap();
        */

        let me: Throw = (match code.chars().nth(2).unwrap() {
            'X' => Ok(
                match opp {Throw::ROCK => Throw::SCISSORS, Throw::PAPER => Throw::ROCK, Throw::SCISSORS => Throw::PAPER}
            ),
            'Y' => Ok(opp.clone()),
            'Z' => Ok(
                match opp {Throw::ROCK => Throw::PAPER, Throw::PAPER => Throw::SCISSORS, Throw::SCISSORS => Throw::ROCK}
            ),
            _ => Err("unable to parse my throw")
        }).unwrap();
        
        Ok(MatchOutcome {me, opp})
    }
}


fn compute_score(m: MatchOutcome) -> i32 {
    let win_value = 6;
    let draw_value = 3;
    let loss_value = 0;
    let vs_score = match m.opp {
        Throw::ROCK => match m.me {Throw::PAPER => win_value, Throw::ROCK => draw_value, Throw::SCISSORS => loss_value},
        Throw::PAPER => match m.me {Throw::SCISSORS => win_value, Throw::PAPER => draw_value, Throw::ROCK => loss_value},
        Throw::SCISSORS => match m.me {Throw::ROCK => win_value, Throw::SCISSORS => draw_value, Throw::PAPER => loss_value},
    };

    let throw_score = match m.me {
        Throw::ROCK => 1,
        Throw::PAPER => 2, 
        Throw::SCISSORS => 3,
    };

    vs_score + throw_score
}


pub fn main(lines: io::Lines<io::BufReader<File>>) {
    let mut score = 0;
    for line in lines {
        if let Ok(code) = line {
            if let Ok(outcome) = MatchOutcome::from_str(&code) {
                score += compute_score(outcome);
            }
        }
    }

    println!("Final score: {}", score);
}