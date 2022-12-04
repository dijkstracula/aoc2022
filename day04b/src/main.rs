fn parse(s: &str) -> ((u32, u32), (u32, u32))
{
    let mut it = s.split(",")
        .map(|r| {
            let mut it = r.split("-").map(|i| i.parse::<u32>().unwrap()).into_iter();
            (it.next().unwrap(), it.next().unwrap())
        })
        .into_iter();
    (it.next().unwrap(), it.next().unwrap())
}

fn overlap(i1: &(u32, u32), i2: &(u32, u32)) -> u32
{
    if i1.0 > i2.0 {
        overlap(i2, i1)
    } else {
        let b = u32::max(i1.0, i2.0);
        let e = u32::min(i1.1, i2.1) + 1;
        u32::saturating_sub(e, b)
    }
}

fn main() {
    println!("{:?}",
        include_str!("../input.txt")
            .lines()
            .map(parse)
            .filter(|(i1, i2)| overlap(i1, i2) > 0)
            .collect::<Vec<_>>()
            .len());
}
