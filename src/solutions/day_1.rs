use regex::{
    self,
    Captures,
    Regex,
};
use std::{
    collections::HashMap,
    fs::read_to_string,
};
pub fn run() {
    // Part 1

    let lines: Vec<String> = read_to_string("../../inputs/day_1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum: u32 = 0;
    //assuming every line has at least one digit

    for line in &lines {
        let char_vec: Vec<char> = line.chars().collect();
        let mut a: usize = 0;
        let mut b: usize = line.len() - 1;

        while !char_vec[a].is_ascii_digit() {
            a += 1
        }

        while !char_vec[b].is_ascii_digit() {
            b -= 1
        }

        let a_num = char_vec[a].to_digit(10).unwrap();
        let b_num = char_vec[b].to_digit(10).unwrap();

        sum += a_num * 10 + b_num
    }

    println!("Day 1 Part 1 solution: {}", sum);

    // Part 2
    sum = 0;
    let reg = Regex::new(r"1|2|3|4|5|6|7|8|9|0|one|two|three|four|five|six|seven|eight|nine|zero")
        .unwrap();

    let rev_reg =
        Regex::new(r"1|2|3|4|5|6|7|8|9|0|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|orez")
            .unwrap();

    let value_map: HashMap<&str, u32> = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
        ("one", 1),
        ("eno", 1),
        ("two", 2),
        ("owt", 2),
        ("three", 3),
        ("eerht", 3),
        ("four", 4),
        ("ruof", 4),
        ("five", 5),
        ("evif", 5),
        ("six", 6),
        ("xis", 6),
        ("seven", 7),
        ("neves", 7),
        ("eight", 8),
        ("thgie", 8),
        ("nine", 9),
        ("enin", 9),
        ("zero", 0),
        ("orez", 0),
    ]
    .iter()
    .cloned()
    .collect();

    for line in &lines {
        // run the forward regex on the original string
        let forward_captures: Vec<Captures> = reg.captures_iter(&line).collect();
        let reversed_string: String = line.chars().rev().collect();
        // run the reverse regex on the reversed string
        let reverse_captures: Vec<Captures> = rev_reg.captures_iter(&reversed_string).collect();
        let (first_forward_capture, []) = forward_captures[0].extract();
        let (first_reverse_capture, []) = reverse_captures[0].extract();

        sum += (value_map.get(first_forward_capture).unwrap() * 10)
            + value_map.get(first_reverse_capture).unwrap();
    }
    println!("Day 1 Part 2 solution: {}", sum);
}
