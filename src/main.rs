use std::io::{stdin, BufRead, BufReader};
use std::ops::Add;

fn main() {
    let mut stream = BufReader::new(stdin());
    let result = compute(&mut stream);
    println!("{}", result);
}

#[derive(Debug)]
enum Direction{
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug)]
struct Position{
    depth: usize,
    forward: usize,
    aim: usize,
}

impl Add<Direction> for Position{
    type Output = Position;
    fn add(self, other: Direction) -> Self{
        match other{
            Direction::Forward(n) => Position{depth: self.depth + (self.aim * n), forward: self.forward + n, aim: self.aim},
            Direction::Down(n) => Position{depth: self.depth, forward: self.forward, aim: self.aim + n},
            Direction::Up(n) => Position{depth: self.depth, forward: self.forward, aim: self.aim - n},
        }
    }
}

fn parse(line: &str) -> Direction{
    let mut parts = line.split(' ');
    let direction = parts.next().unwrap();
    let amount = parts.next().unwrap().parse::<usize>().unwrap();
    match direction {
        "forward" => Direction::Forward(amount),
        "down" => Direction::Down(amount),
        "up" => Direction::Up(amount),
        _ => Direction::Up(0),
    }
}

fn compute<T: BufRead>(mut input: &mut T) -> usize{
    let Position{depth, forward, aim} = input.lines()
        .map(|x|{ parse(&x.unwrap()) }
        )
        .fold(Position{depth:0, forward:0, aim: 0}, |acc, current|{
            acc + current
        });
    depth * forward
}

#[test]
fn test_input(){
   let mut input = "forward 5
down 5
forward 8
up 3
down 8
forward 2".as_bytes();
   assert_eq!(900, compute(&mut input));
}
