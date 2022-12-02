#[derive(PartialEq, Debug)]
enum Choice {
    Rock, Paper, Scissors
}

impl Choice {
    fn score_shape(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3
        }
    }

    fn compute_outcome(&self, them: &Choice) -> Outcome {
        match (self, them) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (x, y) if x == y => Outcome::Draw,
            (_, _) => Outcome::Lose
        }
    }

    fn score(&self, them: &Choice) -> u32 {
        self.score_shape() + self.compute_outcome(them).score_outcome()
    }
}

impl From<&str> for Choice {
    fn from(s: &str) -> Self {
        match s {
           "A" => Choice::Rock, 
           "B" => Choice::Paper, 
           "C" => Choice::Scissors, 
           _ => panic!("Unexpected choice")
        }
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win, Lose, Draw
}

impl Outcome {
    fn score_outcome(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn decide(&self, them: &Choice) -> Choice {
        for us in [Choice::Rock, Choice::Paper, Choice::Scissors].into_iter() {
            if us.compute_outcome(them) == *self {
                return us;
            }
        }
        panic!();
    }
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
           "X" => Outcome::Lose,
           "Y" => Outcome::Draw, 
           "Z" => Outcome::Win, 
           _ => panic!("Unexpected outcome")
        }
    }
}

fn main() {
    println!("{:?}",
        include_str!("../input.txt")
            .lines()
            .map(|l| {
                let mut tok = l.split(" ").into_iter();
                (Choice::from(tok.next().unwrap()), Outcome::from(tok.next().unwrap()))
            })
            .map(|(them, outcome)| (outcome.decide(&them), them))
            .map(|(us, them)| us.score(&them))
            .sum::<u32>());
}
