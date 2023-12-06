use std::{fs, usize};

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("File not found");
    let mut time_vec = vec![];
    let mut distance_vec = vec![];
    for line in input.lines() {
        if line.contains("Time") {
            time_vec.extend(
                line.split_ascii_whitespace()
                    .filter_map(|f| f.parse().ok())
                    .collect::<Vec<usize>>(),
            );
        } else {
            distance_vec.extend(
                line.split_ascii_whitespace()
                    .filter_map(|f| f.parse().ok())
                    .collect::<Vec<usize>>(),
            );
        }
    }
    let first = first_task(&time_vec, &distance_vec);
    println!("Number of ways I could win {first}");
    let second = second_task(&time_vec, &distance_vec);
    println!("Number of ways I could win {second}");
}

fn first_task(time_vec: &[usize], distance_vec: &[usize]) -> usize {
    let mut product = 1;
    for (index, total_time) in time_vec.iter().enumerate() {
        let mut attempt_solutions = 0;
        for attempt in 0..*total_time {
            if attempt * (total_time - attempt) > distance_vec[index] {
                attempt_solutions += 1;
            }
        }
        // println!("attempt {} {total_time}", attempt_solutions);
        product *= attempt_solutions;
    }
    product
}

fn second_task(time_vec: &[usize], distance_vec: &[usize]) -> usize {
    let new_time = time_vec
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let new_distance = distance_vec
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let mut attempt_solutions = 0;
    for attempt in 0..new_time {
        if attempt * (new_time - attempt) > new_distance {
            attempt_solutions += 1;
        }
    }
    attempt_solutions
}
