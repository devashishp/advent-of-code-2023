
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect(
        "Unable to find
    input",
    );
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        matrix.push(line.chars().collect());
    }
    //   println!("{:?}",Matrix);
    let first = first_part(&matrix);
    println!("First part is {first}");
    let second = second_part(&matrix);
    println!("Second part is {second}");
}

fn first_part(input: &Vec<Vec<char>>) -> u32 {
    let mut row = 0;
    let mut sum = 0;
    while row < input.len() {
        let mut column = 0;
        while column < input[0].len() {
            let mut digits = 1;
            match input[row][column] {
                token if token.is_ascii_digit() => {
                    let mut num = token.to_digit(10).unwrap();
                    if column + 2 <= input[row].len()
                        && input[row][column + 1].is_ascii_digit()
                        && input[row][column + 2].is_ascii_digit()
                    {
                        num = num * 100
                            + input[row][column + 1].to_digit(10).unwrap() * 10
                            + input[row][column + 2].to_digit(10).unwrap();
                        digits = 3;
                    } else if column < input[row].len() && input[row][column + 1].is_ascii_digit() {
                        num = num * 10 + input[row][column + 1].to_digit(10).unwrap();
                        digits = 2;
                    }
                    let startrow = if row == 0 { 0 } else { row - 1 };
                    let mut i = startrow;
                    let endrow = match row + 1 {
                        x if x == input.len() => row,
                        _ => row + 1,
                    };
                    while i <= endrow {
                        if i != row {
                            let startcolumn = if column == 0 { 0 } else { column - 1 };
                            let endcolumn = if column + digits + 1 > input[row].len() {
                                column + digits
                            } else {
                                column + digits + 1
                            };
                            for j in startcolumn..endcolumn {
                                if input[i][j].is_ascii_punctuation() && input[i][j] != '.' {
                                    sum += num;
                                }
                            }
                        } else {
                            if column != 0
                                && input[i][column - 1].is_ascii_punctuation()
                                && input[i][column - 1] != '.'
                            {
                                sum += num;
                            }
                            if column + digits < input[i].len()
                                && input[i][column + digits].is_ascii_punctuation()
                                && input[i][column + digits] != '.'
                            {
                                sum += num;
                            }
                        }
                        i += 1;
                    }
                }
                _ => {}
            };
            column += digits;
        }
        row += 1;
    }
    sum
}

fn second_part(input: &Vec<Vec<char>>) -> usize {
    let mut row = 0;
    let mut sum = 0;
    while row < input.len() {
        let mut column = 0;
        while column < input[0].len() {
            let _digits = 1;
            let _exnum = 1;
            if input[row][column] == '*' {
                    let startrow = if row == 0 { 0 } else { row - 1 };
                    let endrow = if row == input.len() - 1 { row } else { row + 1 };
                    let startcolumn = if column == 0 { 0 } else { column - 1 };
                    let endcolumn = if column == input[row].len() - 1 {
                        column
                    } else {
                        column + 1
                    };
                    let mut numsvec: Vec<u32> = vec![];
                    for i in startrow..endrow + 1 {
                        for j in startcolumn..endcolumn + 1 {
                            if input[i][j].is_ascii_digit() {
                                let num = extract_num(input, i, j);
                                if !numsvec.contains(&num) {
                                    numsvec.push(num);
                                }
                            }
                        }
                    }
                    if numsvec.len() == 2 {
                        sum += numsvec.iter().product::<u32>();
                    }
                }
            column += 1;
        }
        row += 1;
    }
    sum as usize
}

fn extract_num(input: &[Vec<char>], i: usize, j: usize) -> u32 {
    // Possibilites : single digit, two digit, three digit, to left and to right
    if !input[i][j - 1].is_ascii_digit() && !input[i][j + 1].is_ascii_digit() {
        return input[i][j].to_digit(10).unwrap();
    }
    if input[i][j - 1].is_ascii_digit() && !input[i][j + 1].is_ascii_digit() {
        if input[i][j - 2].is_ascii_digit() {
            return input[i][j - 2].to_digit(10).unwrap() * 100
                + input[i][j - 1].to_digit(10).unwrap() * 10
                + input[i][j].to_digit(10).unwrap();
        }
        return input[i][j - 1].to_digit(10).unwrap() * 10 + input[i][j].to_digit(10).unwrap();
    }
    if !input[i][j - 1].is_ascii_digit() && input[i][j + 1].is_ascii_digit() {
        if input[i][j + 2].is_ascii_digit() {
            return input[i][j].to_digit(10).unwrap() * 100
                + input[i][j + 1].to_digit(10).unwrap() * 10
                + input[i][j + 2].to_digit(10).unwrap();
        }
        input[i][j].to_digit(10).unwrap() * 10 + input[i][j + 1].to_digit(10).unwrap()
    } else {
        input[i][j - 1].to_digit(10).unwrap() * 100
            + input[i][j].to_digit(10).unwrap() * 10
            + input[i][j + 1].to_digit(10).unwrap()
    }
}
