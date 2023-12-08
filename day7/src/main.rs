use std::{collections::BTreeMap, fs};

#[derive(Debug, Eq)]
struct Hand {
    hand: String,
}
impl Hand {
    fn new(input: &str) -> Self {
        Self {
            hand: input.to_string(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_score = get_score(&self.hand);
        let other_score = get_score(&other.hand);
        if my_score != other_score {
            return my_score.cmp(&other_score);
        }
        let mychars = self.hand.chars().collect::<Vec<_>>();
        let otherchars = other.hand.chars().collect::<Vec<_>>();
        for item in 0..self.hand.len() {
            let my_score = high_card(mychars[item]);
            let other_score = high_card(otherchars[item]);
            if my_score != other_score {
                return my_score.cmp(&other_score);
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("File not found");
    let mut poker: BTreeMap<Hand, usize> = BTreeMap::new();
    for line in input.lines() {
        let current = line.split_whitespace().collect::<Vec<_>>();
        poker.insert(Hand::new(current[0]), current[1].parse::<usize>().unwrap());
    }
    let mut first = 0;
    for (index, item) in poker.values().enumerate() {
        first += (index + 1) * item;
    }
    println!("First answer is {first}");
}

fn high_card(input: char) -> usize {
    #[cfg(feature = "joker")]
    if input == 'J' {
        return 1;
    }
    match input {
        x if x.is_numeric() => x.to_digit(10).unwrap() as usize,
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => 0,
    }
}

#[cfg(not(joker))]
fn get_score(input: &str) -> usize {
    // Cases Five of a kind 6, four of a kind 5, full house 4. three of a kind 3, two of a
    // kind 2, one of a kind 1
    let mut char_map: BTreeMap<char, usize> = BTreeMap::new();
    for ch in input.chars() {
        char_map.entry(ch).and_modify(|f| *f += 1).or_insert(0);
    }
    #[cfg(feature = "joker")]
    if input.contains("J") {
        let jcount = char_map.get(&'J').unwrap().clone() + 1;
        let rank = match char_map.len() {
            5 => 2,
            4 => 4,
            3 => {
                let other = char_map.values().max().unwrap().clone();
                if jcount == 1 {
                    if other == 1 {
                        5
                    } else {
                        6
                    }
                } else {
                    6
                }
            }
            2 => 7,
            1 => 7,
            _ => 0,
        };
        return rank;
    }

    match char_map {
        x if x.len() == 5 => 1, // No common
        x if x.len() == 4 => 2, // 1 common (two of a kind)
        x if x.len() == 3 => {
            // 3 of a kind or two twos

            if x.values().max().unwrap() == &1 {
                // Two pairs
                3
            } else {
                // Three of a kind
                4
            }
        }
        x if x.len() == 2 => {
            if x.values().max().unwrap() == &2 {
                // full house
                5
            } else {
                6
            }
            // Four of a kind
        } // 4 of a kind or full house
        x if x.len() == 1 => 7, // five of a kind
        _ => 0,
    }
}
