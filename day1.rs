use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(file_path: &String) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let letters_regex = Regex::new(r"[A-Za-z]").unwrap();

    let sum: i64 = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| letters_regex.replace_all(&line, "").to_string())
        .map(|line| {
            let first = line.chars().next().unwrap();
            let last = line.chars().last().unwrap();
            let num = format!("{first}{last}");

            return num.parse::<i64>().unwrap();
        })
        .sum();

    println!("part1: {sum}");
}

fn part2(file_path: &String) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<i64> = vec![];

    let number_strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    reader
        .lines()
        .map(|line| line.unwrap().to_lowercase())
        .for_each(|line| {
            let mut digits: Vec<u8> = vec![];
            let mut current_characters = "".to_string();

            for chr in line.chars() {
                if chr >= '1' && chr <= '9' {
                    let chr = chr as u8;
                    digits.push(chr - ('0' as u8));
                    current_characters = "".to_string();
                    continue;
                }

                current_characters.push(chr);

                number_strings
                    .iter()
                    .enumerate()
                    .for_each(|(idx, number_string)| {
                        if current_characters.contains(number_string) {
                            digits.push((idx as u8) + 1);

                            while current_characters.contains(number_string){
                                current_characters.drain(..1);
                            }
                        }
                    });
            }

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            let num = format!("{first}{last}");

            numbers.push(num.parse().unwrap());
        });

    let sum: i64 = numbers.iter().sum();

    println!("part2: {sum}");
}

fn main() {
    let file_path = "input.txt".to_string();

    // part1(&file_path);
    part2(&file_path);
}
