use std::{fs::File, io::Read};

fn main() {
    let file_name = "input.txt";
    let contents = read_file(file_name).expect("Something went wrong reading the file");
    let res = contents.lines().map(get_number).sum::<u64>();
    println!("{}", res);
}

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_number(string: &str) -> u64 {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut number: u64 = 0;
    let mut flag = false;

    // Forward iteration
    for (idx, c) in string.chars().enumerate() {
        if c.is_digit(10) {
            number = c.to_digit(10).unwrap() as u64;
            break;
        } else if c.is_alphabetic() {
            let str = &string[..idx + 1];
            if let Some(d) = digits.iter().find(|&&d| str.contains(d)) {
                number += (digits.iter().position(|&r| r == *d).unwrap() as u64) + 1;
                flag = true;
            }
        }
        if flag {
            break;
        }
    }

    number *= 10;
    flag = false;

    // Reverse iteration
    for (idx, c) in string.chars().rev().enumerate() {
        if c.is_digit(10) {
            number += c.to_digit(10).unwrap() as u64;
            break;
        } else if c.is_alphabetic() {
            let str = &string[(string.len() - idx - 1)..];
            if let Some(d) = digits.iter().find(|&&d| str.contains(d)) {
                number += (digits.iter().position(|&r| r == *d).unwrap() as u64) + 1;
                flag = true;
            }
        }
        if flag {
            break;
        }
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let numbers = vec![12, 38, 15, 77];
        for (idx, line) in lines.iter().enumerate() {
            let number = get_number(line);
            assert_eq!(number, numbers[idx]);
        }
    }

    #[test]
    fn part_02() {
        let lines = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let numbers = vec![29, 83, 13, 24, 42, 14, 76];
        for (idx, line) in lines.iter().enumerate() {
            let number = get_number(line);
            assert_eq!(number, numbers[idx]);
        }
    }
}
