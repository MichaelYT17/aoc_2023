use std::fs::read_to_string;
pub fn run() {
    let lines: Vec<String> = read_to_string("../../inputs/day_1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum: u32 = 0;
    // assuming every line has at least one digit

    for line in lines {
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

    println!("Day 1 solution: {}", sum);
}
