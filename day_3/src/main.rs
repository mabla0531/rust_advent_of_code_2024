// The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!

// It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 
// digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.

// However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like 
// part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

use regex::Regex;

fn main() {
    let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let rx = Regex::new(r"mul\([0-9]+\,[0-9]+\)").unwrap();

    let tokens: Vec<&str> = rx.find_iter(memory).map(|token| token.as_str()).collect();

    let num_pairs: Vec<(i32, i32)> = tokens.iter().map(|token| {
        let comma_separated = &token[4..token.len() - 1];
        let split = comma_separated.split(",").collect::<Vec<&str>>();
        
        (split[0].parse::<i32>().unwrap(), split[1].parse::<i32>().unwrap())

    }).collect();

    assert_eq!(161, num_pairs.iter().map(|pair| pair.0 * pair.1).sum::<i32>());
    println!("Complete!");
}
