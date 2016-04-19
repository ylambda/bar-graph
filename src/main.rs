use std::io;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("read error");

    let data = parse_input(input);
    render(&data);
}

fn parse_input(input: String) -> Vec<i32> {
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().ok().expect("Invalid data set"))
        .collect();

    numbers
}

fn render(data: &Vec<i32>) {
    let mut height = data
        .iter()
        .fold(0, |x, &num| if num > x {num} else {x});

    while height > -1 {
        let line = data.iter()
            .map(|n| if *n > height {"█ "} else {"  "})
            .collect::<String>();
        println!("{}", line);
        height = height - 1;
    }
}
