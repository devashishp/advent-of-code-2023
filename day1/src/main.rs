use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Unable to find
    input");
    let first = first_part(&input);
    println!("First part is {first}");
    let second = second_part(&input);
    println!("Second part is {second}");

}

// Somewhat functional
fn first_part(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let digits =  line.chars().filter(|f|
    f.is_ascii_digit()).map(|f| f.to_digit(10).unwrap()).collect::<Vec<u32>>();
        sum += digits.first().unwrap()*10 + digits.last().unwrap();
    }
    sum

}

// Gave up on functional
fn second_part(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut current_string:String = String::from("");
        let mut first_digit: u32 = 0;
        let dict = ["one","two","three","four","five","six","seven","eight","nine"];
        for i in line.chars() {
            if i.is_ascii_digit() {
                first_digit = i.to_digit(10).unwrap();
                break;
            }
            current_string.push(i);
            for (index, item) in dict.iter().enumerate() {
                if current_string.contains(item){
                    first_digit = index as u32 + 1;
                }
            }
            if first_digit != 0 {
                break;
            }

            
        }
        current_string = String::from("");
        let mut second_digit: u32 = 0;
        for i in line.chars().rev() {
            if i.is_ascii_digit() {
                second_digit = i.to_digit(10).unwrap();
                break;
            }
            let mut temp = String::from(i);
            temp.push_str(&current_string);
            current_string = temp;
            for (index, item) in dict.iter().enumerate() {
                if current_string.contains(item){
                    second_digit = index as u32 + 1;
                }
            }
            if second_digit != 0 {
                break;
            }
        }
        sum += first_digit*10 + second_digit;
    }
    sum

}