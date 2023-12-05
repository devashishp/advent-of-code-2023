use std::fs;

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

macro_rules! get_winner_count {
    ($lines:ident, $index:ident, $winners:ident) => {{
        $lines
            .split('|')
            .filter(|f| !f.contains("Card"))
            .flat_map(|f| f.split_ascii_whitespace())
            .collect::<Vec<_>>()
            .iter()
            .filter_map(|f| f.parse::<u32>().ok())
            .filter(|f| $winners[$index].contains(f))
            .collect::<Vec<u32>>()
            .len()
    }};
}

fn first_part(input: &str) -> usize {
    let winners: Vec<Vec<u32>> = get_winners(input);
    //  println!("{:?}", winners);
    let mut sum = 0;
    for (index, line) in input.lines().enumerate() {
        let count = get_winner_count!(line, index, winners);
        if count != 0 {
            sum += usize::pow(2, (count - 1) as u32);
        }
    }
    sum
}

fn second_part(input: &str) -> u32 {
    let mut keeping_track: Vec<u32> = vec![1; input.lines().count()];
    let winners = get_winners(input);
    for (index, line) in input.lines().enumerate() {
        let count = get_winner_count!(line, index, winners);
        for _j in 0..keeping_track[index] {
            for i in 1..count + 1 {
                if index + i < input.len() {
                    keeping_track[index + i] += 1;
                }
            }
        }
    }
    let sum = keeping_track.iter().sum();
    sum
}

fn get_winners(input: &str) -> Vec<Vec<u32>> {
    let mut winners: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        let current = line
            .split('|')
            .filter(|f| f.contains("Card"))
            .flat_map(|f| f.split_ascii_whitespace())
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|f| f.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        winners.push(current);
    }
    winners
}
