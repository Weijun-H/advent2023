use std::{fs::File, io::Read};

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn lookup(contents: &Vec<&str>, x: i32, y: i32) -> bool {
    x < 0 || y < 0 || y >= contents.len() as i32 || x >= contents[0].len() as i32
}

pub fn is_symbol(contents: &Vec<&str>, x: i32, y: i32) -> bool {
    let directions = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for dir in directions.iter() {
        let mut x = x;
        let mut y = y;
        x += dir.0;
        y += dir.1;

        // check boundaries
        if lookup(contents, x, y) {
            continue;
        }
        // check if it is adjacent to the symbol
        let c = contents[y as usize].chars().nth(x as usize).unwrap();
        if !c.is_ascii_digit() && c != '.' {
            return false;
        }
    }
    true
}

pub fn get_number(line: &str, start: &mut usize) -> Option<i32> {
    for i in *start..line.len() {
        if !line.chars().nth(i).unwrap().is_ascii_digit() {
            let num = line[*start..i].parse::<i32>().unwrap();

            *start = i;
            return Some(num);
        }
    }
    Some(line[*start..].parse::<i32>().unwrap())
}

fn main() {
    let contents = read_file("input.txt").unwrap();
    let contents = contents
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut res: i32 = 0;

    let height = contents.len();
    let width = contents[0].len();

    for y in 0..height {
        let line = contents[y];
        let mut idx = 0;

        while idx < line.len() {
            let mut is_number = false;

            for x in idx..line.len() {
                let c = line.chars().nth(x).unwrap();
                if !c.is_ascii_digit() || is_number {
                    break;
                }
                is_number = !is_symbol(&contents, x as i32, y as i32);
            }

            if is_number {
                if let Some(value) = get_number(line, &mut idx) {
                    res += value;
                }
            }
            idx += 1;
        }
    }

    println!("Part one: {}", res);
}
