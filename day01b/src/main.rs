fn main() {
    let mut foo = include_str!("../input.txt")
            .split("\n\n")
            .map(|lines| lines.lines().map(|l| l.parse::<u32>().unwrap()).sum())
            .collect::<Vec<u32>>();
    foo.sort();
    println!("{}", foo.iter().rev().take(3).sum::<u32>());
}
