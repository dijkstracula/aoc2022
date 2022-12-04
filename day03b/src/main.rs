fn process_group(g: &[&str]) -> char {
    assert!(g.len() == 3);
    g[0].chars().find(|c| g[1].contains(*c) && g[2].contains(*c)).unwrap()
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
        include_str!("../input0.txt")
            .lines()
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(process_group)
            .map(score)
            .sum::<u32>())
}
