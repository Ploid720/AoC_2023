use std::{fs, time::Instant};

fn solve_part_1(input: &String) -> i64 {
    let lines: Vec<Vec<i32>> = input.lines()
        .map(|line| line.split_ascii_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .filter_map(|x| x.parse::<i32>().ok())
            .collect())
        .collect();

    let times = &lines[0];
    let records = &lines[1];

    return times.iter()
        .zip(records)
        .map(|(time, record)| {
            let d = time*time - 4 * record;
            if d < 0 {
                return 0;
            }
            let x1 = ((*time as f64 - (d as f64).sqrt()) / 2.0).ceil() as i64;
            let x2 = ((*time as f64 + (d as f64).sqrt()) / 2.0).floor() as i64;

            let mut ret = x2 - x1 + 1;

            if (x1 * (*time as i64 - x1)) <= *record as i64 {
                ret -= 1;
            }
            if (x2 * (*time as i64 - x2)) <= *record as i64 {
                ret -= 1;
            }

            return ret;
        })
        .reduce(|acc, e| acc * e)
        .unwrap_or(0);
}

fn solve_part_2(input: &String) -> i64 {
    let lines: Vec<i64> = input.lines()
        .map(|x| &x[(x.chars().position(|c| c == ':').unwrap_or(0) + 1)..])
        .map(|x| x.replace(" ", ""))
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let time = &lines[0];
    let record = &lines[1];

    let d = time*time - 4 * record;
    if d < 0 {
        return 0;
    }
    let x1 = ((*time as f64 - (d as f64).sqrt()) / 2.0).ceil() as i64;
    let x2 = ((*time as f64 + (d as f64).sqrt()) / 2.0).floor() as i64;

    let mut ret = x2 - x1 + 1;

    if (x1 * (*time as i64 - x1)) <= *record as i64 {
        ret -= 1;
    }
    if (x2 * (*time as i64 - x2)) <= *record as i64 {
        ret -= 1;
    }

    return ret;
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