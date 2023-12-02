use std::fs;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect(
        "Unable to find
    input",
    );
    let first = first_part(&input);
    println!("First part is {first}");
    let second = second_part(&input);
    println!("Second part is {second}");
}

fn first_part(input: &str) -> usize {
    let temp = input
        .lines()
        .filter(|x| {
            let temp = x
                .split(':')
                .filter(|f| !f.contains("Game"))
                .flat_map(|f| f.split(';'))
                .flat_map(|f| f.split(','))
                .collect::<Vec<_>>();
            let mut valid: bool = true;
            for item in temp.iter() {
                if item.contains("green")
                    && item
                        .split_ascii_whitespace()
                        .filter_map(|f| f.parse::<u32>().ok())
                        .all(|f| f > GREEN)
                {
                    valid = false;
                }

                if item.contains("red")
                    && item
                        .split_ascii_whitespace()
                        .filter_map(|f| f.parse::<u32>().ok())
                        .all(|f| f > RED)
                {
                    valid = false;
                }

                if item.contains("blue")
                    && item
                        .split_ascii_whitespace()
                        .filter_map(|f| f.parse::<u32>().ok())
                        .all(|f| f > BLUE)
                {
                    valid = false;
                }
            }
            valid
        })
        .collect::<Vec<&str>>();

    let mut sum: usize = 0;
    for item in temp {
        let temp2 = item
            .split(':')
            .filter(|f| f.contains("Game"))
            .collect::<Vec<_>>();
        for item in temp2.iter() {
            let game = item
                .split_ascii_whitespace()
                .filter_map(|f| f.parse::<usize>().ok())
                .sum::<usize>();
            sum += game;
            //   println!("Included {game}");
        }
    }

    sum
}

fn second_part(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        let mut rgb = [1, 1, 1];
        let temp = line
            .split(':')
            .filter(|f| !f.contains("Game"))
            .collect::<Vec<&str>>();
        for item in temp {
            let temp2 = item.split(';').collect::<Vec<&str>>();
            for item in temp2 {
                let temp3 = item.split(',').collect::<Vec<&str>>();
                for item in temp3 {
                    if item.contains("red") {
                        let red = item
                            .split_ascii_whitespace()
                            .filter_map(|f| f.parse::<usize>().ok())
                            .reduce(|acc, e| acc + e)
                            .unwrap();
                        if red > rgb[0] {
                            rgb[0] = red;
                        }
                    }
                    if item.contains("green") {
                        let green = item
                            .split_ascii_whitespace()
                            .filter_map(|f| f.parse::<usize>().ok())
                            .reduce(|acc, e| acc + e)
                            .unwrap();
                        if green > rgb[1] {
                            rgb[1] = green;
                        }
                    }
                    if item.contains("blue") {
                        let blue = item
                            .split_ascii_whitespace()
                            .filter_map(|f| f.parse::<usize>().ok())
                            .reduce(|acc, e| acc + e)
                            .unwrap();
                        if blue > rgb[2] {
                            rgb[2] = blue;
                        }
                    }
                }
            }
        }
        sum += rgb[0] * rgb[1] * rgb[2];
    }
    sum
}
