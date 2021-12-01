use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut stream = BufReader::new(stdin());
    let result = compute(&mut stream);
    println!("{}", result);
}

fn compute<T: BufRead>(mut input: &mut T) -> usize{
   let (sum, _) = input.lines().map(|x|{
       x.unwrap().parse::<usize>().unwrap()
    }).fold((0, None), |(sum, prev), current|{
       if let Some(prev) = prev{
        if current > prev{
            return (sum + 1, Some(current))
        }
        else {
            return (sum, Some(current))
        }
       }
    return (0, Some(current))
   });
   sum
}

#[test]
fn test_input(){
   let mut input = "199
200
208
210
200
207
240
269
260
263".as_bytes();
   assert_eq!(7, compute(&mut input));
}
