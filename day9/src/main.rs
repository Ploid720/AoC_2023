use std::{fs, time::Instant};

fn solve_part_1(input: &String) -> i32 {
    return input.lines()
        .map(|line| line.split_ascii_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .filter_map(|x| x.parse::<i32>().ok())
            .fold(Vec::<i32>::new(), |mut stack, mut e| {
                for i in 0..stack.len() {
                    let v = stack[i];
                    stack[i] = e;
                    e = e - v;
                }
                if e != 0 {
                    stack.push(e);
                }
                return stack;
            })
            .iter()
            .sum::<i32>()
        )
        .sum();
}

fn solve_part_2(input: &String) -> i32 {
    return input.lines()
        .map(|line| line.split_ascii_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .filter_map(|x| x.parse::<i32>().ok())
            .rev()
            .fold(Vec::<i32>::new(), |mut stack, mut e| {
                for i in 0..stack.len() {
                    let v = stack[i];
                    stack[i] = e;
                    e = v - e;
                }
                if e != 0 {
                    stack.push(e);
                }
                return stack;
            })
            .iter()
            .map(|e| *e)
            .rev()
            .fold(0,|acc, e| e - acc)
        )
        .sum();
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let inst1 = Instant::now();
    let part_1_res = solve_part_1(&content);
    let t1 = inst1.elapsed();
    let inst2 = Instant::now();
    let part_2_res = solve_part_2(&content);
    let t2 = inst2.elapsed();

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
    println!("Part 2 result: {}, solved in {} ms", part_2_res, t2.as_secs_f64() * 1000.0);
}