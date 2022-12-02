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

    fn adjudicate(&self, them: &Choice) -> Outcome {
        match (self, them) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (x, y) if x == y => Outcome::Draw,
            (_, _) => Outcome::Lose
        }
    }

    fn score(&self, them: &Choice) -> u32 {
        self.score_shape() + self.adjudicate(them).score_outcome()
    }
}

impl From<&str> for Choice {
    fn from(s: &str) -> Self {
        match s {
           "A" | "X" => Choice::Rock, 
           "B" | "Y" => Choice::Paper, 
           "C" | "Z" => Choice::Scissors, 
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
}

fn main() {
    println!("{:?}",
        include_str!("../input.txt")
            .lines()
            .map(|l| {
                let mut tok = l.split(" ").into_iter();
                (Choice::from(tok.next().unwrap()), Choice::from(tok.next().unwrap()))
            })
            .map(|(them, us)| us.score(&them))
            .sum::<u32>());
}
