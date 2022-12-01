fn main() {
    println!("{:?}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|lines| lines.lines().fold(0, |acc, l| acc + l.parse::<i32>().unwrap()))
            .max());
}
