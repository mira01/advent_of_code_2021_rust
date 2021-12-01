use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut stream = BufReader::new(stdin());
    let result = compute(&mut stream);
    println!("result: {:?}", result);
}
fn compute<T: BufRead>(mut input: &mut T) -> usize{
    let numbers: Vec<usize> = input.lines().map(|x|{
        x.unwrap().parse::<usize>().unwrap()
    }).collect();
    let numbers_slice = &numbers[..];
    let (count, _) = numbers_slice.windows(3).fold((0, None), |(count, prev), current|{
        let sum = current[0] + current[1] + current[2];
        if let Some(prev) = prev{
            if sum > prev {
                (count + 1, Some(sum))
            } else {
                (count, Some(sum))
            }
        } else {(0, Some(sum))}
    });
    count
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
    assert_eq!(5, compute(&mut input));
}
