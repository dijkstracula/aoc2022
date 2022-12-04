use std::collections::HashSet;

fn process(line: &str) -> char {
    assert!(line.len() % 2 == 0);
    let r1 = &line[..line.len()/2];
    let r2 = &line[line.len()/2..];

    let mut h1: HashSet<char> = HashSet::new();
    let mut h2: HashSet<char> = HashSet::new();
    for c in r1.chars() {
        h1.insert(c);
    }
    for c in r2.chars() {
        h2.insert(c);
    }

    let intr : Vec<&char> = h1.intersection(&h2).collect();
    assert!(intr.len() == 1);

    *intr[0]
}

fn score(c: char) -> u32 {
    if c.is_uppercase() {
        27 + (c as u32 - 'A' as u32)
    } else {
        1 + (c as u32 - 'a' as u32)
    }
}

fn main() {
    println!("{:?}",
        include_str!("../input.txt")
            .lines()
            .map(process)
            .map(score)
            .sum::<u32>())
}
