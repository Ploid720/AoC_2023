use std::fs;
use std::cmp;
use std::collections::HashMap;

fn solve_part_1(input: &String) -> i32 {
    let arr = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn check_for_symbols(parser_pos: usize, num_len: usize, line_ind: usize, arr: &Vec<Vec<char>>) -> bool {
        let min_x = cmp::max(parser_pos as isize - num_len as isize - 1, 0) as usize;
        let max_x = cmp::min(parser_pos + 1, arr[line_ind].len());
        let min_y = cmp::max(line_ind as isize - 1, 0) as usize;
        let max_y = cmp::min(line_ind + 2, arr.len());

        for x in min_x..max_x {
            for y in min_y..max_y {
                match arr[y][x] {
                    '0'..='9' => {}
                    '.' => {}
                    _ => return true
                }
            }
        }

        return false;
    }

    return input.lines()
        .enumerate()
        .map(|(line_ind, line)| {
            let mut num = String::new();
            let mut ret = 0;
            for (i, c) in line.chars().enumerate() {
                match c {
                    ('0'..='9') => num.push(c),
                    _ => {
                        let num_len = num.len();
                        if num_len < 1 {
                            continue;
                        }
                        let valid = check_for_symbols(i, num_len, line_ind, &arr);
                        if !valid {
                            num.clear();
                            continue;
                        }
                        ret += num.parse::<i32>().unwrap_or(0);
                        num.clear();
                    }
                }
            }
            let valid = check_for_symbols(line.len(), num.len(), line_ind, &arr);
            if valid {
                ret += num.parse::<i32>().unwrap_or(0);
            }

            return ret;
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
}

fn solve_part_2(input: &String) -> i32 {
    let arr = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut gears: HashMap<(usize, usize), (i32, i32)> = HashMap::new();

    fn push_to_symbols(parser_pos: usize, num: &String, line_ind: usize, arr: &Vec<Vec<char>>, gears: &mut HashMap<(usize, usize), (i32, i32)>) {
        let num_len = num.len();
        let min_x = cmp::max(parser_pos as isize - num_len as isize - 1, 0) as usize;
        let max_x = cmp::min(parser_pos + 1, arr[line_ind].len());
        let min_y = cmp::max(line_ind as isize - 1, 0) as usize;
        let max_y = cmp::min(line_ind + 2, arr.len());
        let num_val = num.parse::<i32>().unwrap_or(0);

        for x in min_x..max_x {
            for y in min_y..max_y {
                match arr[y][x] {
                    '*' => {
                        let curr = gears.get(&(x, y)).unwrap_or(&(0, 1));
                        gears.insert((x, y), (curr.0 + 1, curr.1 * num_val));
                    }
                    _ => {}
                }
            }
        }
    }

    input.lines()
        .enumerate()
        .for_each(|(line_ind, line)| {
            let mut num = String::new();
            for (i, c) in line.chars().enumerate() {
                match c {
                    ('0'..='9') => num.push(c),
                    _ => {
                        if num.len() < 1 {
                            continue;
                        }
                        push_to_symbols(i, &num, line_ind, &arr, &mut gears);
                        num.clear();
                    }
                }
            }
            push_to_symbols(line.len(), &num, line_ind, &arr, &mut gears);
        });

    return gears.iter()
        .filter_map(|e| 
        if e.1.0 == 2 {
            Some(e.1.1)
        }
        else {
            None
        })
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