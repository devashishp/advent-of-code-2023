use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to file file");
    let parsed: Vec<Vec<isize>> = input
        .lines()
        .map(|f| {
            f.split_whitespace()
                .map(|f| f.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<_>>();

    println!("Sum of first is {}", get_sum(&parsed, false));
    println!("Sum of second is {}", get_sum(&parsed, true));
}

fn get_sum(input: &[Vec<isize>], second: bool) -> isize {
    input
        .iter()
        .map(|f| {
            let mut result_vec: Vec<isize> = if !second {
                vec![*f.iter().last().unwrap()]
            } else {
                vec![*f.iter().next().unwrap()]
            };
            let mut temp = f.clone();
            loop {
                temp = temp.windows(2).map(|w| w[1] - w[0]).collect();
                if !second {
                    result_vec.push(temp.last().unwrap().to_owned());
                } else {
                    result_vec.push(temp.iter().next().unwrap().to_owned());
                }
                if temp.iter().all(|f| f == &0) {
                    break;
                }
            }
            let individual_sum = if !second {
                result_vec.iter().sum::<isize>()
            } else {
                result_vec
                    .into_iter()
                    .rev()
                    .reduce(|acc, e| e - acc)
                    .unwrap()
            };
            individual_sum
        })
        .sum()
}
