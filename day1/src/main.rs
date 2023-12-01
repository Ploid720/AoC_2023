
use std::fs;

fn solve_part_1(input: &String) -> i32
{
    return input.lines()
        .map(|line| {
            let mut first_found = false;
            let mut first = '0';
            let mut last = '0';
            for c in line.chars()
            {
                if (c >= '0') && (c <= '9')
                {
                    if !first_found {
                        first_found = true;
                        first = c;
                    }
                    last = c;
                }
            }
            if first_found {
                return format!("{}{}", first, last).parse::<i32>().ok();
            }
            else {
                return None;
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
}

fn solve_part_2(input: &String) -> i32
{
    let digit_map = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ].map(|x| (x.0.as_bytes(), x.1));

    fn match_digit(arr: &[u8], i: usize, digit_map: &[(&[u8], u8)]) -> Option<u8>
    {
        let l = arr.len();
        for digit in digit_map {
            let digit_name = digit.0;
            let dl = digit_name.len();
            if i + dl > l {
                continue;
            }
            let mut digit_matches = true;
            for j in 0..dl {
                if arr[i + j] != digit_name[j] {
                    digit_matches = false;
                    break;
                }
            }
            if digit_matches {
                return Some(digit.1);
            }
        }
        return None;
    }

    return input.lines()
        .map(|line| {
            let mut first_found = false;
            let mut first = 0;
            let mut last = 0;

            let arr = line.as_bytes();
            let l = arr.len();
            for i in 0..l {
                let c = arr[i];
                match c {
                    b'0'..=b'9' => {
                        if !first_found {
                            first_found = true;
                            first = c - b'0';
                        }
                        last = c - b'0';
                    }
                    _ => {
                        let digit_match = match_digit(arr, i, &digit_map);
                        if digit_match.is_none() {
                            continue;
                        }

                        let digit_value: u8 = digit_match.unwrap();
                        if !first_found {
                            first_found = true;
                            first = digit_value;
                        }
                        last = digit_value;
                    }
                }
            }
            if first_found {
                return format!("{}{}", first, last).parse::<i32>().ok();
            }
            else {
                return None;
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let part_1_res = solve_part_1(&content);
    let part_2_res = solve_part_2(&content);

    println!("Part 1 result: {}", part_1_res);
    println!("Part 2 result: {}", part_2_res);
}
