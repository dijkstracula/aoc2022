use itertools::Itertools;

fn parse_diagram(lines: &[&str]) -> Vec<Vec<char>>
{
    let mut ret: Vec<Vec<char>> = Vec::new();

    for l in lines.iter().rev() {
        let crates = l.chars().chunks(4);
        for (i, mut c) in crates.into_iter().enumerate() {
            if c.next().unwrap() == ' ' {
                continue;
            }
            let c = c.next().unwrap();
            while ret.len() <= i {
                ret.push(Vec::new());
            }
            ret[i].push(c);
        }
    }

    ret
}

#[derive(Debug)]
struct Op {
    rep: usize,
    src: usize,
    dst: usize,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        assert!(s.starts_with("move "));
        let toks = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let rep = str::parse::<usize>(toks[1]).unwrap();
        let src = str::parse::<usize>(toks[3]).unwrap() - 1;
        let dst = str::parse::<usize>(toks[5]).unwrap() - 1;
        Op {rep, src, dst }
    }
}

fn parse_ops(lines: &[&str]) -> Vec<Op>
{
    let mut ret: Vec<Op> = Vec::new();
    for l in lines.into_iter() {
        ret.push((*l).into());
    }
    ret
}

fn operate(state: &mut Vec<Vec<char>>, ops: Vec<Op>)
{
    for o in ops {
        for _ in 0..o.rep {
            let c = state[o.src].pop().unwrap();
            state[o.dst].push(c);
        }
    }
}

fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let (diagram, operations) = lines.split_at(lines.iter().find_position(|l| l.starts_with(" 1")).unwrap().0);

    let mut state = parse_diagram(diagram);
    operate(&mut state, parse_ops(&operations[2..]));

    println!("{:?}", 
        state
            .iter()
            .filter(|v| !v.is_empty())
            .map(|v| v.last().unwrap())
            .collect::<String>());
}
