use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("File not found");
    let input_vec = input
        .lines()
        .take(1)
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|f| f.chars())
        .collect::<Vec<char>>();
    let mut map: BTreeMap<String, (String, String)> = BTreeMap::new();
    for line in input.lines() {
        if line.contains('=') {
            let key_value = line
                .split('=')
                .collect::<Vec<_>>()
                .iter()
                .map(|f| f.trim())
                .collect::<Vec<_>>();
            let key = key_value[0];
            let value = key_value[1]
                .split(',')
                .collect::<Vec<_>>()
                .iter()
                .map(|f| f.trim_matches('(').trim_matches(')').trim())
                .collect::<Vec<_>>();
            map.insert(
                key.to_string(),
                (value[0].to_string(), value[1].to_string()),
            );
        }
    }
    // Soln

    println!("Took {} steps", first_task(&input_vec, &map));
    println!("Took {} steps", second_task(&input_vec, &map));
}

fn first_task(input_vec: &[char], map: &BTreeMap<String, (String, String)>) -> usize {
    let mut key = "AAA".to_string();
    let mut steps = 0;
    loop {
        steps += 1;
        if input_vec[(steps - 1) % input_vec.len()] == 'L' {
            key = map.get(&key).unwrap().0.to_string();
        } else {
            key = map.get(&key).unwrap().1.to_string();
        }
        if key == "ZZZ" {
            break;
        }
    }
    steps
}

fn second_task(input_vec: &[char], map: &BTreeMap<String, (String, String)>) -> usize {
    let keys = map
        .keys()
        .filter(|f| f.ends_with('A'))
        .map(|f| f.to_owned())
        .collect::<Vec<_>>();
    let mut answers_vec: Vec<usize> = vec![];
    let _final_string: String = "AAA".to_string();
    for item in keys {
        let mut counter = 0;
        let mut current = item.clone();
        loop {
            counter += 1;
            let current_num = (counter - 1) % input_vec.len();
            if input_vec[current_num] == 'L' {
                current = map.get(&current).unwrap().0.to_string();
            } else {
                current = map.get(&current).unwrap().1.to_string();
            };
            if current.ends_with('Z') {
                answers_vec.push(counter);
                break;
            }
        }
    }
    lcm(&answers_vec)
}

// lcm Code from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(my_list: &[usize]) -> usize {
    if my_list.len() == 1 {
        return my_list[0];
    }
    let a = my_list[0];
    let b = lcm(&my_list[1..]);
    a * b / gcd(a, b)
}
