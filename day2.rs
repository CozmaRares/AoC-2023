use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_games(file_path: &String) -> Vec<(i32, Vec<Set>)> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let game_regex = Regex::new(r"^Game (\d+): (.+?)$").unwrap();
    let outcome_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut games: Vec<(i32, Vec<Set>)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let capture = game_regex.captures(&line).unwrap();

        let id = capture
            .get(1)
            .map(|m| m.as_str())
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let sets = capture
            .get(2)
            .map(|m| {
                m.as_str()
                    .split(";")
                    .map(|s| {
                        s.split(",")
                            .map(|s| s.trim().to_string())
                            .map(|s| {
                                let capture = outcome_regex.captures(&s).unwrap();
                                let number_cubes =
                                    capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                                let color = capture.get(2).unwrap().as_str().to_string();

                                return (number_cubes, color);
                            })
                            .collect::<Vec<(i32, String)>>()
                    })
                    .map(|set| {
                        let mut cubes: HashMap<String, i32> = HashMap::new();
                        cubes.insert("red".to_string(), 0);
                        cubes.insert("green".to_string(), 0);
                        cubes.insert("blue".to_string(), 0);

                        for (number_cubes, color) in set {
                            *cubes.entry(color.clone()).or_insert(0) += number_cubes;
                        }

                        Set {
                            red: *cubes.get("red").unwrap_or(&0),
                            green: *cubes.get("green").unwrap_or(&0),
                            blue: *cubes.get("blue").unwrap_or(&0),
                        }
                    })
                    .collect::<Vec<Set>>()
            })
            .unwrap();

        games.push((id, sets));
    }

    return games;
}

fn part1(file_path: &String) {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let games = parse_games(file_path);

    let sum: i32 = games
        .iter()
        .filter(|(_, game)| {
            game.iter()
                .all(|set| set.red <= max_red && set.green <= max_green && set.blue <= max_blue)
        })
        .map(|game| game.0)
        .sum();

    println!("part1: {sum}");
}

fn part2(file_path: &String) {
    let games = parse_games(file_path);

    let sum: i32 = games
        .iter()
        .map(|(_, game)| {
            game.iter().fold(
                Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, set| {
                    acc.red = i32::max(acc.red, set.red);
                    acc.green = i32::max(acc.green, set.green);
                    acc.blue = i32::max(acc.blue, set.blue);
                    return acc;
                },
            )
        })
        .map(|Set { red, green, blue }| red * green * blue)
        .sum();

    println!("part2: {:?}", sum);
}

fn main() {
    let file_path = "input.txt".to_string();

    // part1(&file_path);
    part2(&file_path);
}
