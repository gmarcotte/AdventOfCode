use std::io;
use std::fs::File;
use std::str::FromStr;

#[derive(Clone,Debug,PartialEq)]
enum Throw {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Throw {
    fn score(&self) -> i32 {
        use Throw::*;
        match *self {
            ROCK => 1,
            PAPER => 2,
            SCISSORS => 3,
        }
    }

    fn beats(&self) -> Throw {
        use Throw::*;
        match *self {
            ROCK => SCISSORS,
            PAPER => ROCK,
            SCISSORS => PAPER,
        }
    }

    fn beaten_by(&self) -> Throw {
        use Throw::*;
        match *self {
            ROCK => PAPER,
            PAPER => SCISSORS,
            SCISSORS => ROCK,
        }
    }
}

enum MatchOutcome {
    WIN,
    DRAW,
    LOSS,
}

impl MatchOutcome {
    fn score(&self) -> i32 {
        use MatchOutcome::*;
        match *self {WIN => 6, DRAW => 3, LOSS => 0}
    }
}

#[derive(Debug)]
struct RPSMatch {
    me: Throw,
    opp: Throw,
}

impl RPSMatch {
    fn outcome(&self) -> MatchOutcome {
        use MatchOutcome::*;
        if self.me == self.opp.beats() {
            return LOSS;
        } else if self.me == self.opp.beaten_by() {
            return WIN;
        } else {
            return DRAW;
        }
    }

    fn my_score(&self) -> i32 {
        self.outcome().score() + self.me.score()
    }

}

impl FromStr for RPSMatch {
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
            'X' => Ok(opp.beats()),
            'Y' => Ok(opp.clone()),
            'Z' => Ok(opp.beaten_by()),
            _ => Err("unable to parse my throw")
        }).unwrap();
        
        Ok(RPSMatch {me, opp})
    }
}


pub fn main(lines: io::Lines<io::BufReader<File>>) {
    let mut score = 0;
    for line in lines {
        if let Ok(code) = line {
            if let Ok(rps_match) = RPSMatch::from_str(&code) {
                score += rps_match.my_score();
            }
        }
    }

    println!("Final score: {}", score);
}